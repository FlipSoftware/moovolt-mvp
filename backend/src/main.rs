use std::panic;
use std::net::SocketAddr;

use axum::extract::ws::Message as AxumWSMessage;
use axum::extract::ConnectInfo;
use axum::routing::get;
use axum::Router;
use axum_extra::TypedHeader;
use chrono::{DateTime, Utc};
use dotenvy_macro::dotenv;
use futures::{SinkExt, StreamExt};
use rust_ocpp::v1_6::messages::{authorize::AuthorizeRequest, boot_notification::{BootNotificationRequest, BootNotificationResponse}, cancel_reservation::CancelReservationRequest, change_availability::ChangeAvailabilityRequest, change_configuration::ChangeConfigurationRequest, clear_cache::ClearCacheRequest, clear_charging_profile::ClearChargingProfileRequest, data_transfer::DataTransferRequest, diagnostics_status_notification::DiagnosticsStatusNotificationRequest, firmware_status_notification::FirmwareStatusNotificationRequest, get_composite_schedule::GetCompositeScheduleRequest, get_diagnostics::GetDiagnosticsRequest, get_local_list_version::GetLocalListVersionRequest, heart_beat::HeartbeatRequest, meter_values::MeterValuesRequest, remote_start_transaction::RemoteStartTransactionRequest, remote_stop_transaction::RemoteStopTransactionRequest, reserve_now::ReserveNowRequest, reset::ResetRequest, send_local_list::SendLocalListRequest, set_charging_profile::SetChargingProfileRequest, status_notification::StatusNotificationRequest, trigger_message::TriggerMessageRequest, unlock_connector::UnlockConnectorRequest, update_firmware::UpdateFirmwareRequest};
use tokio::{net, sync::OnceCell};
use tracing::{event, info, Level};

type OcppMessageTypeId = usize;
type OcppMessageId = String;
type OcppErrorCode = String;
type OcppErrorDescription = String;
type OcppErrorDetails = serde_json::Value;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum OcppActionEnum {
    // OCPP 1.6 JSON
    // Core
    Authorize,
    BootNotification,
    ChangeAvailability,
    ChangeConfiguration , // TODO: new 1.6 core
    DataTransfer,
    ClearCache,
    GetConfiguration, // TODO: new 1.6 core
    Heartbeat,
    MeterValues,
    RemoteStartTransaction, // TODO: new 1.6 core
    RemoteStopTransaction, // TODO: new 1.6 core
    Reset,
    StatusNotification,
    StartTransaction,
    StopTransaction,
    UnlockConnector,
    // Firmware
    DiagnosticsStatusNotification, // TODO: new 1.6 firmware
    FirmwareStatusNotification,
    GetDiagnostics, // TODO: new 1.6 firmware
    UpdateFirmware,
    // Local Authorization
    GetLocalListVersion,
    SendLocalList,
    // Remote Trigger
    CancelReservation,
    ReserveNow,
    // Reservation
    ClearChargingProfile,
    GetCompositeSchedule,
    SetChargingProfile, // TODO: new 1.6 firmware
    // Smart Charging
    TriggerMessage,
    // TODO: OCPP 2.0.1 JSON
    // TODO: Identify OCPP 2.0.1 and 1.6 versions to switch Enums and Messages pragrammatically
    // CertificateSigned,
    // ClearDisplayMessage,
    // ClearedChargingLimit,
    // ClearVariableMonitoring,
    // CostUpdated,
    // CustomerInformation,
    // DeleteCertificate,
    // Get15118EVCertificate,
    // GetBaseReport,
    // GetCertificateStatus,
    // GetChargingProfile,
    // GetDisplayMessage,
    // GetInstalledCertificateIds,
    // GetLog,
    // GetMonitoringReport,
    // GetReport,
    // GetTransactionStatus,
    // GetVariables,
    // InstallCertificate,
    // LogStatusNotification,
    // NotifyChargingLimit,
    // NotifyCustomerInformation,
    // NotifyDisplayMessages,
    // NotifyEVChargingNeeds,
    // NotifyEVChargingSchedule,
    // NotifyEvent,
    // NotifyMonitoringReport,
    // NotifyReport,
    // PublishFirmware,
    // PublishFirmwareStatusNotification,
    // ReportChargingProfiles,
    // RequestStartTransaction,
    // RequestStopTransaction,
    // ReservationStatusUpdate,
    // SecurityEventNotification,
    // SetDisplayMessage,
    // SetMonitoringBase,
    // SetMonitoringLevel,
    // SetNetworkProfile,
    // SetVariableMonitoring,
    // SetVariables,
    // SignCertificate,
    // TransactionEvent,
    // UnpublishFirmware,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum OcppPayload {
    // OCPP 1.6 JSON
    // Core
    Authorize(AuthorizeRequest),
    BootNotification(BootNotificationRequest),
    ChangeAvailability(ChangeAvailabilityRequest),
    ChangeConfiguration(ChangeConfigurationRequest), // TODO: new 1.6 core
    ClearCache(ClearCacheRequest),
    DataTransfer(DataTransferRequest),
    GetConfiguration, // TODO: new 1.6 core
    Heartbeat(HeartbeatRequest),
    MeterValues(MeterValuesRequest),
    RemoteStartTransaction(RemoteStartTransactionRequest), // TODO: new 1.6 core
    RemoteStopTransaction(RemoteStopTransactionRequest), // TODO: new 1.6 core
    // RequestStartTransaction(StartTransactionRequest), // TODO: remove and test
    // RequestStopTransaction(StopTransactionRequest), // TODO: remove and test
    Reset(ResetRequest),
    StatusNotification(StatusNotificationRequest),
    UnlockConnector(UnlockConnectorRequest),
    // Firmware
    DiagnosticsStatusNotification(DiagnosticsStatusNotificationRequest),
    FirmwareStatusNotification(
        FirmwareStatusNotificationRequest,
    ),
    GetDiagnostics(GetDiagnosticsRequest), // TODO: new 1.6 firmware
    UpdateFirmware(UpdateFirmwareRequest),
    // Local Authorization
    GetLocalListVersion(
        GetLocalListVersionRequest,
    ),
    SendLocalList(SendLocalListRequest),
    // Remote Trigger
    CancelReservation(CancelReservationRequest),
    ReserveNow(ReserveNowRequest),
    // Reservation
    ClearChargingProfile(
        ClearChargingProfileRequest,
    ),
    GetCompositeSchedule(
        GetCompositeScheduleRequest,
    ),
    SetChargingProfile(SetChargingProfileRequest),
    // Smart Chargin
    TriggerMessage(TriggerMessageRequest),
    // TODO: Identify OCPP 2.0.1 and 1.6 versions to switch Enums and Messages pragrammatically
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

static TIME_NOW: OnceCell<DateTime<Utc>> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    async fn time_now() -> DateTime<Utc> {
        Utc::now()
    }
    let time_now = TIME_NOW.get_or_init(time_now).await;
    
    tracing_subscriber::fmt().with_max_level(Level::DEBUG).init();

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
        .route("/", get(healthcheck_route));

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
                    match &call {
                        OcppMessageType::Call(_, _, action, _) => event!(Level::DEBUG, ACTION_TYPE = action),
                        _ => (),
                    }
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

async fn healthcheck_route() -> impl axum::response::IntoResponse {
    if let Some(time) = TIME_NOW.get() {
        axum::response::Html::from(
            format!("<h1>Server working. Started at: {time}</h1>")
        )
    } else {
        axum::response::Html::from(
            format!("<h1>Server not started yet</h1>")
        )
    }
}

fn default_error_response() -> BootNotificationResponse {
    BootNotificationResponse {
        status: rust_ocpp::v1_6::types::RegistrationStatus::Rejected,
        current_time: Utc::now(),
        interval: 60,
    }
}
