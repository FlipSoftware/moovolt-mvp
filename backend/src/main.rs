use std::{net::SocketAddr, panic, str::FromStr};

use axum::{
    extract::{ws::Message as AxumWSMessage, ConnectInfo},
    routing::get,
    Router,
};
use axum_extra::TypedHeader;
use chrono::Utc;
use dotenvy_macro::dotenv;
use futures::StreamExt;
use rust_ocpp::v1_6::messages::{
    authorize::{AuthorizeRequest, AuthorizeResponse},
    boot_notification::{self, BootNotificationRequest, BootNotificationResponse},
    cancel_reservation::CancelReservationRequest,
    change_availability::ChangeAvailabilityRequest,
    change_configuration::{ChangeConfigurationRequest, ChangeConfigurationResponse},
    clear_cache::ClearCacheRequest,
    clear_charging_profile::ClearChargingProfileRequest,
    data_transfer::DataTransferRequest,
    diagnostics_status_notification::DiagnosticsStatusNotificationRequest,
    firmware_status_notification::FirmwareStatusNotificationRequest,
    get_composite_schedule::GetCompositeScheduleRequest,
    get_configuration::{GetConfigurationRequest, GetConfigurationResponse},
    get_diagnostics::GetDiagnosticsRequest,
    get_local_list_version::GetLocalListVersionRequest,
    heart_beat::HeartbeatRequest,
    meter_values::MeterValuesRequest,
    remote_start_transaction::RemoteStartTransactionRequest,
    remote_stop_transaction::RemoteStopTransactionRequest,
    reserve_now::ReserveNowRequest,
    reset::ResetRequest,
    send_local_list::SendLocalListRequest,
    set_charging_profile::SetChargingProfileRequest,
    start_transaction::{StartTransactionRequest, StartTransactionResponse},
    status_notification::StatusNotificationRequest,
    trigger_message::TriggerMessageRequest,
    unlock_connector::UnlockConnectorRequest,
    update_firmware::UpdateFirmwareRequest,
};
use strum_macros::Display;
use tokio::{net, sync::OnceCell};
use tracing::{debug, error, event, info, warn, Level};

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
    ChangeConfiguration,
    DataTransfer,
    ClearCache,
    GetConfiguration,
    Heartbeat,
    MeterValues,
    RemoteStartTransaction,
    RemoteStopTransaction,
    Reset,
    StatusNotification,
    StartTransaction,
    StopTransaction,
    UnlockConnector,
    // // Firmware
    // DiagnosticsStatusNotification,
    // FirmwareStatusNotification,
    // GetDiagnostics,
    // UpdateFirmware,
    // // Local Authorization
    // GetLocalListVersion,
    // SendLocalList,
    // // Remote Trigger
    // CancelReservation,
    // ReserveNow,
    // // Reservation
    // ClearChargingProfile,
    // GetCompositeSchedule,
    // SetChargingProfile,
    // // Smart Charging
    // TriggerMessage,
}

impl FromStr for OcppActionEnum {
    type Err = String;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "Authorize" => Ok(Self::Authorize),
            "BootNotification" => Ok(Self::BootNotification),
            "ChangeAvailability" => Ok(Self::ChangeAvailability),
            "ChangeConfiguration" => Ok(Self::ChangeConfiguration),
            "ClearCache" => Ok(Self::ClearCache),
            "DataTransfer" => Ok(Self::DataTransfer),
            "GetConfiguration" => Ok(Self::GetConfiguration),
            "Heartbeat" => Ok(Self::Heartbeat),
            "MeterValues" => Ok(Self::MeterValues),
            "RemoteStartTransaction" => Ok(Self::RemoteStartTransaction),
            "RemoteStopTransaction" => Ok(Self::RemoteStopTransaction),
            "Reset" => Ok(Self::Reset),
            "StatusNotification" => Ok(Self::StatusNotification),
            "StartTransaction" => Ok(Self::StartTransaction),
            "StopTransaction" => Ok(Self::StopTransaction),
            "UnlockConnector" => Ok(Self::UnlockConnector),
            // "DiagnosticsStatusNotification" => Ok(Self::DiagnosticsStatusNotification),
            // "FirmwareStatusNotification" => Ok(Self::FirmwareStatusNotification),
            // "GetDiagnostics" => Ok(Self::GetDiagnostics),
            // "UpdateFirmware" => Ok(Self::UpdateFirmware),
            // "GetLocalListVersion" => Ok(Self::GetLocalListVersion),
            // "SendLocalList" => Ok(Self::SendLocalList),
            // "CancelReservation" => Ok(Self::CancelReservation),
            // "ReserveNow" => Ok(Self::ReserveNow),
            // "ClearChargingProfile" => Ok(Self::ClearChargingProfile),
            // "GetCompositeSchedule" => Ok(Self::GetCompositeSchedule),
            // "SetChargingProfile" => Ok(Self::SetChargingProfile),
            // "TriggerMessage" => Ok(Self::TriggerMessage),
            _ => Err(format!("Unknown OCPP action: {str}")),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
enum AuthorizeKind {
    Request(AuthorizeRequest),
    Response(AuthorizeResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
enum BootNotificationKind {
    Request(BootNotificationRequest),
    Response(BootNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
enum ChangeAvailabilityKind {
    Request(ChangeAvailabilityRequest),
    Response(ChangeAvailabilityRequest),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
enum ChangeConfigurationKind {
    Request(ChangeConfigurationRequest),
    Response(ChangeConfigurationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
enum ClearCacheKind {
    Request(ClearCacheRequest),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
enum DataTransferKind {
    Request(DataTransferRequest),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
enum GetConfigurationKind {
    Request(GetConfigurationRequest),
    Response(GetConfigurationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
enum HeartbeatKind {
    Request(HeartbeatRequest),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
enum MeterValuesKind {
    Request(MeterValuesRequest),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
enum RemoteStartTransactionKind {
    Request(RemoteStartTransactionRequest),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
enum RemoteStopTransactionKind {
    Request(RemoteStopTransactionRequest),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
enum ResetKind {
    Request(ResetRequest),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
enum StartTransactionKind {
    Request(StartTransactionRequest),
    Response(StartTransactionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
enum StatusNotificationKind {
    Request(StatusNotificationRequest),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
enum UnlockConnectorKind {
    Request(UnlockConnectorRequest),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum OcppPayload {
    // OCPP 1.6 JSON
    // Core
    Authorize(AuthorizeKind),                     // Charger → Server
    BootNotification(BootNotificationKind),       // Charger → Server
    ChangeAvailability(ChangeAvailabilityKind),    // Server → Charger
    ChangeConfiguration(ChangeConfigurationKind), // Server → Charger
    ClearCache(ClearCacheKind),                    // Server → Charger
    DataTransfer(DataTransferKind),                // Both Directions
    GetConfiguration(GetConfigurationKind),       // Server → Charger
    Heartbeat(HeartbeatKind),                      // Charger → Server
    MeterValues(MeterValuesKind),                  // Charger → Server
    RemoteStartTransaction(RemoteStartTransactionKind), // Server → Charger
    RemoteStopTransaction(RemoteStopTransactionKind), // Server → Charger
    Reset(ResetKind),                              // Server → Charger
    StartTransaction(StartTransactionKind),       // Charger → Server
    StatusNotification(StatusNotificationKind),    // Charger → Server
    StopTransaction(StatusNotificationKind),       // Charger → Server
    UnlockConnector(UnlockConnectorKind),          // Server → Charger
    // // Firmware
    // DiagnosticsStatusNotification(DiagnosticsStatusNotificationRequest), // Charger → Server
    // FirmwareStatusNotification(FirmwareStatusNotificationRequest),       // Charger → Server
    // GetDiagnostics(GetDiagnosticsRequest),                               // Server → Charger
    // UpdateFirmware(UpdateFirmwareRequest),                               // Server → Charger
    // // Local Authorization
    // GetLocalListVersion(GetLocalListVersionRequest), // Server → Charger
    // SendLocalList(SendLocalListRequest),             // Server → Charger
    // // Remote Trigger
    // CancelReservation(CancelReservationRequest), // Server → Charger
    // ReserveNow(ReserveNowRequest),               // Server → Charger
    // // Reservation
    // ClearChargingProfile(ClearChargingProfileRequest), // Server → Charger
    // GetCompositeSchedule(GetCompositeScheduleRequest), // Server → Charger
    // SetChargingProfile(SetChargingProfileRequest),     // Server → Charger
    // // Smart Chargin
    // TriggerMessage(TriggerMessageRequest), // Server → Charger
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// Call: [<MessageTypeId>, "<MessageId>", "<Action>", {<Payload>}]
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
/// CallError: [<MessageTypeId>, "<MessageId>", "<errorCode>", "<errorDescription>",
/// {<errorDetails>}]
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

static TIME_NOW: OnceCell<String> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    async fn time_now() -> String {
        let date_time = Utc::now();
        let formatted = format!("{}", date_time.format("%d/%m/%Y %H:%M"));
        formatted
    }
    let _time_now = TIME_NOW
        .get_or_init(time_now)
        .await;

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

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
        .route("/ocpp16j/:station_id", get(upgrade_to_ws))
        .route("/", get(healthcheck_route));

    // Start the Axum server
    axum::serve(
        tcp_listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Failed to start server");
}

// Upgrade from a HTTP connection to a WebSocket connection
#[axum::debug_handler]
async fn upgrade_to_ws(
    ws: axum::extract::WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl axum::response::IntoResponse {
    // Check if the user agent is a valid OCPP client
    match user_agent {
        Some(user_agent) => {
            if user_agent.as_str() == "OCPP" {
                info!("User agent is a valid OCPP client: {user_agent:?}");
            } else {
                warn!("User agent is not a valid OCPP client: {user_agent:?}");
            }
        },
        None => warn!("User agent is not present"),
    }
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(mut socket: axum::extract::ws::WebSocket, addr: SocketAddr) {
    info!("New WebSocket connection: {addr}");

    while let Some(Ok(msg)) = socket
        .next()
        .await
    {
        match msg {
            AxumWSMessage::Text(text) => {
                let message = text.clone();
                info!("Received text message: {message} \nfrom: {addr}");
                handle_ocpp_messages(text, &mut socket).await;
            },
            AxumWSMessage::Binary(_) => warn!("Unexpected binary message"),
            AxumWSMessage::Close(_) => info!("WebSocket connection closed"),
            _ => (),
        }
    }
}

// Handle the incoming WebSocket connections and their OCPP Messages
async fn handle_ocpp_messages(message: String, socket: &mut axum::extract::ws::WebSocket) {
    // Try to parse the JSON message
    match serde_json::from_str(&message) {
        Ok(ocpp_message) => {
            debug!("Parsed OCPP message: {ocpp_message:?}");
            //
            match ocpp_message {
                OcppMessageType::Call(message_type_id, message_id, action, payload) => {
                    let action = match OcppActionEnum::from_str(&action) {
                        Ok(action) => {
                            debug!("Parsed OCPP Action: {action:?}");
                            action
                        },
                        Err(err) => {
                            warn!("Failed to parse OCPP Action: {err:?}");
                            return;
                        },
                    };
                    handle_ocpp_call(message_type_id, message_id, action, payload, socket).await;
                },
                OcppMessageType::CallResult(message_type_id, message_id, payload) => {
                    handle_ocpp_call_result(message_type_id, message_id, payload, socket).await;
                },
                OcppMessageType::CallError(
                    message_type_id,
                    message_id,
                    error_code,
                    error_description,
                    error_details,
                ) => {
                    handle_ocpp_call_error(
                        message_type_id,
                        message_id,
                        error_code,
                        error_description,
                        error_details,
                        socket,
                    )
                    .await;
                },
            }
        },
        Err(err) => {
            warn!("Failed to parse OCPP message: {err:?}");
            return;
        },
    }
}

// Handle the incoming OCPP Call messages
async fn handle_ocpp_call(
    message_type_id: OcppMessageTypeId,
    message_id: OcppMessageId,
    action: OcppActionEnum,
    payload: serde_json::Value,
    socket: &mut axum::extract::ws::WebSocket,
) {
    let payload = match serde_json::from_value::<OcppPayload>(payload) {
        Ok(ocpp_payload) => {
            debug!("Parsed OCPP Payload: {ocpp_payload:?}");
            ocpp_payload
        },
        Err(err) => {
            error!("Failed to parse OCPP Payload: {err:?}");
            return;
        }
    };
    // Handle the OCPP Action
    use OcppActionEnum::*;
    match action {
        Authorize => {
            info!("Handling OCPP Authorize action");
        },
        BootNotification => {
            info!("Handling OCPP BootNotification action");
            match payload {
                OcppPayload::BootNotification(BootNotificationKind::Request(boot_notification)) => {
                    if boot_notification.charge_point_serial_number == Some("NKYK430037668".to_string()) {
                        info!("Validating OCPP BootNotification: {boot_notification:?}");
                    } else {
                        warn!("Invalid OCPP BootNotification: {boot_notification:?}");
                    }
                },
                _ => ()
            }
            let response = OcppCallResult {
                message_type_id: 3, // 3 = CallResult or CallError sent by the server
                message_id,
                payload: OcppPayload::BootNotification(BootNotificationKind::Response(BootNotificationResponse {
                    status: rust_ocpp::v1_6::types::RegistrationStatus::Accepted,
                    current_time: Utc::now(),
                    interval: 60,
                })),
            };
            let response_json = serde_json::to_string(&response).unwrap();
            info!("Sending OCPP BootNotification: {response_json}");
            socket
                .send(axum::extract::ws::Message::Text(response_json))
                .await
                .unwrap();
        },
        ChangeAvailability => {
            info!("Handling OCPP ChangeAvailability action");
        },
        ChangeConfiguration => {
            info!("Handling OCPP ChangeConfiguration action");
        },
        ClearCache => {
            info!("Handling OCPP ClearCache action");
        },
        DataTransfer => {
            info!("Handling OCPP DataTransfer action");
        },
        GetConfiguration => {
            info!("Handling OCPP GetConfiguration action");
        },
        Heartbeat => {
            info!("Handling OCPP Heartbeat action");
        },
        MeterValues => {
            info!("Handling OCPP MeterValues action");
        },
        RemoteStartTransaction => {
            info!("Handling OCPP RemoteStartTransaction action");
        },
        RemoteStopTransaction => {
            info!("Handling OCPP RemoteStopTransaction action");
        },
        Reset => {
            info!("Handling OCPP Reset action");
        },
        StatusNotification => {
            info!("Handling OCPP StatusNotification action");
        },
        StartTransaction => {
            info!("Handling OCPP StartTransaction action");
        },
        StopTransaction => {
            info!("Handling OCPP StopTransaction action");
        },
        UnlockConnector => {
            info!("Handling OCPP UnlockConnector action");
        },
    }
}

// Handle the incoming OCPP CallResult messages
async fn handle_ocpp_call_result(
    message_type_id: OcppMessageTypeId,
    message_id: OcppMessageId,
    payload: serde_json::Value,
    socket: &mut axum::extract::ws::WebSocket,
) {
    match serde_json::from_value::<OcppPayload>(payload) {
        Ok(ocpp_payload) => {
            info!("Parsed OCPP Payload: {ocpp_payload:?}");
        },
        Err(err) => {
            warn!("Failed to parse OCPP Payload: {err:?}");
        },
    }
}

// Handle the incoming OCPP CallError messages
async fn handle_ocpp_call_error(
    message_type_id: OcppMessageTypeId,
    message_id: OcppMessageId,
    error_code: String,
    error_description: String,
    error_details: serde_json::Value,
    socket: &mut axum::extract::ws::WebSocket,
) {
    let ocpp_call_error = OcppCallError {
        message_type_id,
        message_id,
        error_code,
        error_description,
        error_details,
    };
    let ocpp_call_error_json = serde_json::to_string(&ocpp_call_error).unwrap();
    info!("Sending OCPP CallError: {ocpp_call_error_json}");
    socket
        .send(axum::extract::ws::Message::Text(ocpp_call_error_json))
        .await
        .unwrap();
}

async fn healthcheck_route() -> impl axum::response::IntoResponse {
    if let Some(time) = TIME_NOW.get() {
        axum::response::Html::from(format!("<h1>Server working. Started at: {time}</h1>"))
    } else {
        axum::response::Html::from(format!("<h1>Server has not started yet</h1>"))
    }
}

// fn default_error_response() -> BootNotificationResponse {
//     BootNotificationResponse {
//         status: rust_ocpp::v1_6::types::RegistrationStatus::Rejected,
//         current_time: Utc::now(),
//         interval: 60,
//     }
// }
