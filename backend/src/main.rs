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
use owo_colors::OwoColorize;
use rust_ocpp::v1_6::messages::{
    authorize::{AuthorizeRequest, AuthorizeResponse},
    boot_notification::{BootNotificationRequest, BootNotificationResponse},
    change_availability::ChangeAvailabilityRequest,
    change_configuration::{ChangeConfigurationRequest, ChangeConfigurationResponse},
    clear_cache::{ClearCacheRequest, ClearCacheResponse},
    data_transfer::{DataTransferRequest, DataTransferResponse},
    get_configuration::{GetConfigurationRequest, GetConfigurationResponse},
    heart_beat::{HeartbeatRequest, HeartbeatResponse},
    meter_values::{MeterValuesRequest, MeterValuesResponse},
    remote_start_transaction::{RemoteStartTransactionRequest, RemoteStartTransactionResponse},
    remote_stop_transaction::{RemoteStopTransactionRequest, RemoteStopTransactionResponse},
    reset::{ResetRequest, ResetResponse},
    start_transaction::{StartTransactionRequest, StartTransactionResponse},
    status_notification::{StatusNotificationRequest, StatusNotificationResponse},
    stop_transaction::{StopTransactionRequest, StopTransactionResponse},
    unlock_connector::{UnlockConnectorRequest, UnlockConnectorResponse},
};
use strum_macros::Display;
use tokio::{net, sync::OnceCell};
use tracing::{debug, error, info, warn, Level};

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
            _ => Err(format!("Unknown OCPP action: {str}")),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum AuthorizeKind {
    Request(AuthorizeRequest),
    Response(AuthorizeResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum BootNotificationKind {
    Request(BootNotificationRequest),
    Response(BootNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ChangeAvailabilityKind {
    Request(ChangeAvailabilityRequest),
    Response(ChangeAvailabilityRequest),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ChangeConfigurationKind {
    Request(ChangeConfigurationRequest),
    Response(ChangeConfigurationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ClearCacheKind {
    Request(ClearCacheRequest),
    Response(ClearCacheResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum DataTransferKind {
    Request(DataTransferRequest),
    Response(DataTransferResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetConfigurationKind {
    Request(GetConfigurationRequest),
    Response(GetConfigurationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum HeartbeatKind {
    Request(HeartbeatRequest),
    Response(HeartbeatResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum MeterValuesKind {
    Request(MeterValuesRequest),
    Response(MeterValuesResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum RemoteStartTransactionKind {
    Request(RemoteStartTransactionRequest),
    Response(RemoteStartTransactionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum RemoteStopTransactionKind {
    Request(RemoteStopTransactionRequest),
    Response(RemoteStopTransactionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ResetKind {
    Request(ResetRequest),
    Response(ResetResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum StartTransactionKind {
    Request(StartTransactionRequest),
    Response(StartTransactionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum StopTransactionKind {
    Request(StopTransactionRequest),
    Response(StopTransactionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum StatusNotificationKind {
    Request(StatusNotificationRequest),
    Response(StatusNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum UnlockConnectorKind {
    Request(UnlockConnectorRequest),
    Response(UnlockConnectorResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum OcppPayload {
    // OCPP 1.6 JSON
    // Core
    Authorize(AuthorizeKind),                           // Charger → Server
    BootNotification(BootNotificationKind),             // Charger → Server
    ChangeAvailability(ChangeAvailabilityKind),         // Server → Charger
    ChangeConfiguration(ChangeConfigurationKind),       // Server → Charger
    ClearCache(ClearCacheKind),                         // Server → Charger
    DataTransfer(DataTransferKind),                     // Both Directions
    GetConfiguration(GetConfigurationKind),             // Server → Charger
    Heartbeat(HeartbeatKind),                           // Charger → Server
    MeterValues(MeterValuesKind),                       // Charger → Server
    RemoteStartTransaction(RemoteStartTransactionKind), // Server → Charger
    RemoteStopTransaction(RemoteStopTransactionKind),   // Server → Charger
    Reset(ResetKind),                                   // Server → Charger
    StartTransaction(StartTransactionKind),             // Charger → Server
    StatusNotification(StatusNotificationKind),         // Charger → Server
    StopTransaction(StopTransactionKind),               // Charger → Server
    UnlockConnector(UnlockConnectorKind),               // Server → Charger
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
    let _time_now = TIME_NOW.get_or_init(time_now).await;

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
async fn upgrade_to_ws(
    ws: axum::extract::WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl axum::response::IntoResponse {
    // Check if the user agent is a valid client
    match user_agent {
        Some(TypedHeader(agent)) => {
            if agent.as_str() == "Websocket Client" {
                info!("{agent} user agent is a valid client");
            } else {
                warn!("User agent {agent} is not a valid Websocket Client");
            }
        },
        None => warn!("User agent is not present. Continue without specific platform check"),
    }
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(mut socket: axum::extract::ws::WebSocket, addr: SocketAddr) {
    info!(
        "{} {addr}",
        "New WebSocket connection:"
            .green()
            .bold()
    );

    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            AxumWSMessage::Text(text) => {
                let message = text.clone();
                info!(
                    "\n\t{0}\n\t{1}\n\t\t{message}\n{2} {3}\n\n",
                    "INCOMING CALL".truecolor(255, 255, 255),
                    "FROM CHARGER".truecolor(180, 180, 180),
                    " ADDR ".on_truecolor(0, 115, 0),
                    addr.truecolor(0, 215, 0)
                );
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
        Ok(ocpp_message) => match ocpp_message {
            OcppMessageType::Call(message_type_id, message_id, action, payload) => {
                let action = match OcppActionEnum::from_str(&action) {
                    Ok(action) => {
                        debug!(
                            "\n{0}\n {1}",
                            " PARSED OCPP CALL "
                                .on_truecolor(0, 0, 0)
                                .bold(),
                            format!(" {:?} ", action).on_truecolor(139, 0, 139)
                        );
                        action
                    },
                    Err(err) => {
                        error!("Failed to parse OCPP Call Action: {err:?}");
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
        },
        Err(err) => {
            warn!("Failed to parse OCPP message: {err:?}");
            return;
        },
    }
}

// Handle the incoming OCPP Call messages
async fn handle_ocpp_call(
    _: OcppMessageTypeId,
    message_id: OcppMessageId,
    action: OcppActionEnum,
    payload: serde_json::Value,
    socket: &mut axum::extract::ws::WebSocket,
) {
    let payload = match serde_json::from_value::<OcppPayload>(payload) {
        Ok(ocpp_payload) => ocpp_payload,
        Err(err) => {
            error!("Failed to parse OCPP Payload: {err:?}");
            return;
        },
    };
    // Handle the OCPP Call Action
    use OcppActionEnum::*;
    match action {
        Authorize => {
            match payload {
                OcppPayload::Authorize(AuthorizeKind::Request(authorize)) => {
                    info!(
                        "\n{0}\n {1}\n{authorize:?}",
                        " CALL ".on_truecolor(0, 0, 0).bold(),
                        " REQUEST ".on_truecolor(0, 99, 255)
                    );
                    let response = OcppCallResult {
                        message_type_id: 3,
                        message_id,
                        payload: OcppPayload::Authorize(AuthorizeKind::Response(
                            AuthorizeResponse {
                                id_tag_info: rust_ocpp::v1_6::types::IdTagInfo {
                                    status: rust_ocpp::v1_6::types::AuthorizationStatus::Accepted,
                                    expiry_date: None,
                                    parent_id_tag: None,
                                },
                            },
                        )),
                    };
                    let response_json = serde_json::to_string(&response).unwrap();
                    info!(
                        "\n{0}\n {1}\n{response_json:?}",
                        " CALL RESULT "
                            .on_truecolor(0, 0, 0)
                            .bold(),
                        " RESPONSE ".on_truecolor(0, 125, 0)
                    );
                    socket
                        .send(axum::extract::ws::Message::Text(response_json))
                        .await
                        .unwrap();
                },
                _ => (),
            }
        },
        BootNotification => {
            match payload {
                OcppPayload::BootNotification(BootNotificationKind::Request(boot_notification)) => {
                    if boot_notification.charge_point_serial_number
                        == Some("NKYK430037668".to_string())
                    {
                        info!(
                            "\n{0}\n {1}\n{boot_notification:?}",
                            " CALL ".on_truecolor(0, 0, 0).bold(),
                            " REQUEST ".on_truecolor(0, 99, 255)
                        );
                        let response = OcppCallResult {
                            message_type_id: 3,
                            message_id,
                            payload: OcppPayload::BootNotification(BootNotificationKind::Response(
                                BootNotificationResponse {
                                    status: rust_ocpp::v1_6::types::RegistrationStatus::Accepted,
                                    current_time: Utc::now(),
                                    interval: 300,
                                },
                            )),
                        };
                        let response_json = serde_json::to_string(&response).unwrap();
                        info!(
                            "\n{0}\n {1}\n{response_json:?}",
                            " CALL RESULT "
                                .on_truecolor(0, 0, 0)
                                .bold(),
                            " RESPONSE ".on_truecolor(0, 125, 0)
                        );
                        socket
                            .send(axum::extract::ws::Message::Text(response_json))
                            .await
                            .unwrap();
                    } else {
                        error!(
                            "Invalid Charger Serial Number. BootNotification: \
                             {boot_notification:?}"
                        );
                    }
                },
                _ => error!("Invalid OCPP BootNotification payload"),
            }
        },
        ChangeAvailability => {
        },
        ChangeConfiguration => {
        },
        ClearCache => {
        },
        DataTransfer => {
            match payload {
                OcppPayload::DataTransfer(DataTransferKind::Request(data_transfer)) => {
                    info!(
                        "\n{0}\n {1}\n{data_transfer:?}",
                        " CALL ".on_truecolor(0, 0, 0).bold(),
                        " REQUEST ".on_truecolor(0, 99, 255)
                    );
                    let response = OcppCallResult {
                        message_type_id: 3,
                        message_id,
                        payload: OcppPayload::DataTransfer(DataTransferKind::Response(
                            DataTransferResponse {
                                status: rust_ocpp::v1_6::types::DataTransferStatus::Accepted,
                                data: Some("Data Transfer Accepted".to_string()),
                            },
                        )),
                    };
                    let response_json = serde_json::to_string(&response).unwrap();
                    info!(
                        "\n{0}\n {1}\n{response_json:?}",
                        " CALL RESULT "
                            .on_truecolor(0, 0, 0)
                            .bold(),
                        " RESPONSE ".on_truecolor(0, 125, 0)
                    );
                    socket
                        .send(axum::extract::ws::Message::Text(response_json))
                        .await
                        .unwrap();
                },
                _ => (),
            }
        },
        GetConfiguration => {
        },
        Heartbeat => {
            match payload {
                OcppPayload::Heartbeat(HeartbeatKind::Request(heartbeat)) => {
                    info!(
                        "\n{0}\n {1}\n{heartbeat:?}",
                        " CALL ".on_truecolor(0, 0, 0).bold(),
                        " REQUEST ".on_truecolor(0, 99, 255)
                    );
                    let response = OcppCallResult {
                        message_type_id: 3,
                        message_id,
                        payload: OcppPayload::Heartbeat(HeartbeatKind::Response(
                            HeartbeatResponse { current_time: Utc::now() },
                        )),
                    };
                    let response_json = serde_json::to_string(&response).unwrap();
                    info!(
                        "\n{0}\n {1}\n{response_json:?}",
                        " CALL RESULT "
                            .on_truecolor(0, 0, 0)
                            .bold(),
                        " RESPONSE ".on_truecolor(0, 125, 0)
                    );
                    socket
                        .send(axum::extract::ws::Message::Text(response_json))
                        .await
                        .unwrap();
                },
                _ => (),
            }
        },
        MeterValues => {
        },
        RemoteStartTransaction => {
        },
        RemoteStopTransaction => {
        },
        Reset => {
        },
        StatusNotification => {
            match payload {
                OcppPayload::StatusNotification(StatusNotificationKind::Request(
                    status_notification,
                )) => {
                    info!(
                        "\n{0}\n {1}\n{status_notification:#?}",
                        " CALL ".on_truecolor(0, 0, 0).bold(),
                        " REQUEST ".on_truecolor(0, 99, 255)
                    );
                },
                _ => (),
            }
        },
        StartTransaction => {
        },
        StopTransaction => {
            match payload {
                OcppPayload::StopTransaction(StopTransactionKind::Request(stop_transaction)) => {
                    info!(
                        "\n{0}\n {1}\n{stop_transaction:?}",
                        " CALL ".on_truecolor(0, 0, 0).bold(),
                        " REQUEST ".on_truecolor(0, 99, 255)
                    );
                    let response = OcppCallResult {
                        message_type_id: 3,
                        message_id,
                        payload: OcppPayload::StopTransaction(StopTransactionKind::Response(
                            StopTransactionResponse {
                                id_tag_info: Some(rust_ocpp::v1_6::types::IdTagInfo {
                                    status: rust_ocpp::v1_6::types::AuthorizationStatus::Accepted,
                                    expiry_date: None,
                                    parent_id_tag: None,
                                }),
                            },
                        )),
                    };
                    let response_json = serde_json::to_string(&response).unwrap();
                    info!(
                        "\n{0}\n {1}\n{response_json:?}",
                        " CALL RESULT "
                            .on_truecolor(0, 0, 0)
                            .bold(),
                        " RESPONSE ".on_truecolor(0, 125, 0)
                    );
                    socket
                        .send(axum::extract::ws::Message::Text(response_json))
                        .await
                        .unwrap();
                },
                _ => (),
            }
        },
        UnlockConnector => {
        },
    }
}

// Handle the incoming OCPP CallResult messages
async fn handle_ocpp_call_result(
    _: OcppMessageTypeId,
    _: OcppMessageId,
    payload: serde_json::Value,
    _: &mut axum::extract::ws::WebSocket,
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
