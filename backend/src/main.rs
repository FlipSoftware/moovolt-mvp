use std::panic;
use std::net::SocketAddr;

use axum::extract::ws::Message as AxumWSMessage;
use axum::extract::ConnectInfo;
use axum::routing::get;
use axum::Router;
use axum_extra::TypedHeader;
use chrono::Utc;
use dotenvy_macro::dotenv;
use futures::{SinkExt, StreamExt};
use rust_ocpp::v1_6::messages::boot_notification::{
    BootNotificationRequest, BootNotificationResponse,
};
use tokio::net;
use tracing::{info, warn};

type OcppMessageTypeId = usize;
type OcppMessageId = String;
type OcppErrorCode = String;
type OcppErrorDescription = String;
type OcppErrorDetails = serde_json::Value;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum OcppActionEnum {
    Authorize,
    BootNotification,
    CancelReservation,
    CertificateSigned,
    ChangeAvailability,
    ClearCache,
    ClearChargingProfile,
    ClearDisplayMessage,
    ClearedChargingLimit,
    ClearVariableMonitoring,
    CostUpdated,
    CustomerInformation,
    DataTransfer,
    DeleteCertificate,
    FirmwareStatusNotification,
    Get15118EVCertificate,
    GetBaseReport,
    GetCertificateStatus,
    GetChargingProfile,
    GetCompositeSchedule,
    GetDisplayMessage,
    GetInstalledCertificateIds,
    GetLocalListVersion,
    GetLog,
    GetMonitoringReport,
    GetReport,
    GetTransactionStatus,
    GetVariables,
    Heartbeat,
    InstallCertificate,
    LogStatusNotification,
    MeterValues,
    NotifyChargingLimit,
    NotifyCustomerInformation,
    NotifyDisplayMessages,
    NotifyEVChargingNeeds,
    NotifyEVChargingSchedule,
    NotifyEvent,
    NotifyMonitoringReport,
    NotifyReport,
    PublishFirmware,
    PublishFirmwareStatusNotification,
    ReportChargingProfiles,
    RequestStartTransaction,
    RequestStopTransaction,
    ReservationStatusUpdate,
    ReserveNow,
    Reset,
    SecurityEventNotification,
    SendLocalList,
    SetChargingProfile,
    SetDisplayMessage,
    SetMonitoringBase,
    SetMonitoringLevel,
    SetNetworkProfile,
    SetVariableMonitoring,
    SetVariables,
    SignCertificate,
    StatusNotification,
    TransactionEvent,
    TriggerMessage,
    UnlockConnector,
    UnpublishFirmware,
    UpdateFirmware,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum OcppPayload {
    Authorize(rust_ocpp::v1_6::messages::authorize::AuthorizeRequest),
    BootNotification(BootNotificationRequest),
    CancelReservation(rust_ocpp::v1_6::messages::cancel_reservation::CancelReservationRequest),
    ChangeAvailability(rust_ocpp::v1_6::messages::change_availability::ChangeAvailabilityRequest),
    ClearCache(rust_ocpp::v1_6::messages::clear_cache::ClearCacheRequest),
    ClearChargingProfile(
        rust_ocpp::v1_6::messages::clear_charging_profile::ClearChargingProfileRequest,
    ),
    DataTransfer(rust_ocpp::v1_6::messages::data_transfer::DataTransferRequest),
    FirmwareStatusNotification(
        rust_ocpp::v1_6::messages::firmware_status_notification::FirmwareStatusNotificationRequest,
    ),
    GetCompositeSchedule(
        rust_ocpp::v1_6::messages::get_composite_schedule::GetCompositeScheduleRequest,
    ),
    GetLocalListVersion(
        rust_ocpp::v1_6::messages::get_local_list_version::GetLocalListVersionRequest,
    ),
    Heartbeat(rust_ocpp::v1_6::messages::heart_beat::HeartbeatRequest),
    LogStatusNotification(
        rust_ocpp::v1_6::messages::status_notification::StatusNotificationRequest,
    ),
    MeterValues(rust_ocpp::v1_6::messages::meter_values::MeterValuesRequest),
    RequestStartTransaction(rust_ocpp::v1_6::messages::start_transaction::StartTransactionRequest),
    RequestStopTransaction(rust_ocpp::v1_6::messages::stop_transaction::StopTransactionRequest),
    ReserveNow(rust_ocpp::v1_6::messages::reserve_now::ReserveNowRequest),
    Reset(rust_ocpp::v1_6::messages::reset::ResetRequest),
    SendLocalList(rust_ocpp::v1_6::messages::send_local_list::SendLocalListRequest),
    SetChargingProfile(rust_ocpp::v1_6::messages::set_charging_profile::SetChargingProfileRequest),
    StatusNotification(rust_ocpp::v1_6::messages::status_notification::StatusNotificationRequest),
    TriggerMessage(rust_ocpp::v1_6::messages::trigger_message::TriggerMessageRequest),
    UnlockConnector(rust_ocpp::v1_6::messages::unlock_connector::UnlockConnectorRequest),
    UpdateFirmware(rust_ocpp::v1_6::messages::update_firmware::UpdateFirmwareRequest),
}

/// Call: [<MessageTypeId>, "<MessageId>", "<Action>", {<Payload>}]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct OcppCall {
    pub message_type_id: OcppMessageTypeId,
    pub message_id: OcppMessageId,
    pub action: OcppActionEnum,
    pub payload: OcppPayload,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// CallResult: [<MessageTypeId>, "<MessageId>", {<Payload>}]
pub struct OcppCallResult {
    pub message_type_id: OcppMessageTypeId,
    pub message_id: OcppMessageId,
    pub payload: OcppPayload,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// CallError: [<MessageTypeId>, "<MessageId>", "<errorCode>", "<errorDescription>", {<errorDetails>}]
pub struct OcppCallError {
    pub message_type_id: OcppMessageTypeId,
    pub message_id: OcppMessageId,
    pub error_code: OcppErrorCode,
    pub error_description: OcppErrorDescription,
    pub error_details: OcppErrorDetails,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum OcppMessageType {
    /// OCPP Call
    Call(usize, String, String, serde_json::Value),
    /// OCPP Result
    CallResult(usize, String, serde_json::Value),
    /// OCPP Error
    CallError(usize, String, String, String, serde_json::Value),
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // Get some useful errors before the application ends with panic
    panic::set_hook(Box::new(|err| {
        tracing::error!("\n\nPanic: {err:#?}\n\n");
    }));

    // The server will listen on
    const ADDR: &str = dotenv!("ADDR");
    const PORT: &str = dotenv!("PORT");
    let tcp_listener = net::TcpListener::bind(format!("{ADDR}:{PORT}"))
        .await
        .expect(&format!("Failed to bind to address: {ADDR}"));
    info!("Server listening on {ADDR}:{PORT}");

    // Create the Axum router
    let router = Router::new()
        .route("/ocpp16j/NKYK430037668", get(upgrade_to_ws))
        .route("/", get(hello_route));

    // Start the Axum server
    axum::serve(
        tcp_listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Failed to start server");
}

async fn handle_boot_notification(
    boot_notification_request: BootNotificationRequest,
) -> BootNotificationResponse {
    match boot_notification_request.charge_point_serial_number {
        Some(serial_number) => {
            tracing::info!("BootNotificationRequest received from charge box with serial number: {serial_number}");
            match serial_number.as_str() {
                "NKYK430037668" => {
                    tracing::info!("Charge box serial number is valid");
                    let response = BootNotificationResponse {
                        status: rust_ocpp::v1_6::types::RegistrationStatus::Accepted,
                        current_time: Utc::now(),
                        interval: 60,
                    };
                    response
                }
                _ => {
                    tracing::error!("Charge box serial number is invalid");
                    BootNotificationResponse {
                        status: rust_ocpp::v1_6::types::RegistrationStatus::Rejected,
                        current_time: Utc::now(),
                        interval: 60,
                    }
                }
            }
        }
        None => {
            tracing::error!("BootNotificationRequest received without charge point serial number");
            BootNotificationResponse {
                status: rust_ocpp::v1_6::types::RegistrationStatus::Rejected,
                current_time: Utc::now(),
                interval: 60,
            }
        }
    }
}

// Upgrade from a HTTP connection to a WebSocket connection
#[axum::debug_handler]
async fn upgrade_to_ws(
    ws: axum::extract::WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl axum::response::IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_ws(socket, addr))
}

// Handle the incoming WebSocket connections
async fn handle_ws(socket: axum::extract::ws::WebSocket, who: SocketAddr) {
    let (mut tx, mut rx) = socket.split();

    // Receive messages from the client
    while let Some(Ok(message)) = rx.next().await {
        match message {
            AxumWSMessage::Text(text) => {
                info!("Received message from {who}: {text:?}");
                // Deserialize the received JSON into OcppMessageType and then get the Payload, in this case, BootNotificationRequest
                if let Ok(call) = serde_json::from_str::<OcppMessageType>(&text) {
                    // Handle the BootNotificationRequest
                    warn!("BootNotificationRequest: {call:?}");
                    let response = match call {
                        OcppMessageType::Call(_, _, _, payload) => {
                            // let action = OcppActionEnum::from_str("BootNotification").unwrap();
                            let boot_notification_request = match serde_json::from_value::<
                                BootNotificationRequest,
                            >(
                                payload
                            ) {
                                Ok(request) => request,
                                Err(_) => {
                                    tracing::error!("Failed to parse BootNotificationRequest from client message");
                                    BootNotificationRequest::default()
                                }
                            };
                            handle_boot_notification(boot_notification_request).await
                        }
                        _ => {
                            tracing::error!("Received unsupported message type from {who}");
                            default_error_response()
                        }
                    };
                    // Send the response back to the client
                    tx.send(AxumWSMessage::Text(
                        serde_json::to_string(&response).unwrap(),
                    ))
                    .await
                    .expect("Failed to send response to client");
                } else {
                    tracing::error!("Failed to parse BootNotificationRequest from client message");
                    // Respond with an error if the message cannot be parsed
                    tx.send(AxumWSMessage::Text(
                        serde_json::to_string(&default_error_response()).unwrap(),
                    ))
                    .await
                    .expect("Failed to send response to client");
                }
            }
            AxumWSMessage::Close(_) => {
                info!("Client {who} closed the connection");
                break;
            }
            _ => {
                tracing::warn!("Received unsupported message type from {who}");
            }
        }
    }
}

async fn hello_route() -> impl axum::response::IntoResponse {
    axum::response::Html::from("<h1>Hello World</h1>")
}

fn default_error_response() -> BootNotificationResponse {
    BootNotificationResponse {
        status: rust_ocpp::v1_6::types::RegistrationStatus::Rejected,
        current_time: Utc::now(),
        interval: 60,
    }
}
