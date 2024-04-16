#[inline]
pub unsafe fn MsQuicClose(quicapi: *const ::core::ffi::c_void) {
    #[link(name = "msquic")]
    extern "system" {
        pub fn MsQuicClose(quicapi: *const ::core::ffi::c_void);
    }
    MsQuicClose(quicapi)
}
#[inline]
pub unsafe fn MsQuicOpenVersion(
    version: u32,
    quicapi: *mut *mut ::core::ffi::c_void,
) -> ::windows_core::Result<()> {
    #[link(name = "msquic")]
    extern "system" {
        pub fn MsQuicOpenVersion(
            version: u32,
            quicapi: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    MsQuicOpenVersion(version, quicapi).ok()
}
pub const QUIC_ALLOWED_CIPHER_SUITE_AES_128_GCM_SHA256: QUIC_ALLOWED_CIPHER_SUITE_FLAGS =
    QUIC_ALLOWED_CIPHER_SUITE_FLAGS(1i32);
pub const QUIC_ALLOWED_CIPHER_SUITE_AES_256_GCM_SHA384: QUIC_ALLOWED_CIPHER_SUITE_FLAGS =
    QUIC_ALLOWED_CIPHER_SUITE_FLAGS(2i32);
pub const QUIC_ALLOWED_CIPHER_SUITE_CHACHA20_POLY1305_SHA256: QUIC_ALLOWED_CIPHER_SUITE_FLAGS =
    QUIC_ALLOWED_CIPHER_SUITE_FLAGS(4i32);
pub const QUIC_ALLOWED_CIPHER_SUITE_NONE: QUIC_ALLOWED_CIPHER_SUITE_FLAGS =
    QUIC_ALLOWED_CIPHER_SUITE_FLAGS(0i32);
pub const QUIC_API_VERSION_1: u32 = 1u32;
pub const QUIC_API_VERSION_2: u32 = 2u32;
pub const QUIC_CERTIFICATE_HASH_STORE_FLAG_MACHINE_STORE: QUIC_CERTIFICATE_HASH_STORE_FLAGS =
    QUIC_CERTIFICATE_HASH_STORE_FLAGS(1i32);
pub const QUIC_CERTIFICATE_HASH_STORE_FLAG_NONE: QUIC_CERTIFICATE_HASH_STORE_FLAGS =
    QUIC_CERTIFICATE_HASH_STORE_FLAGS(0i32);
pub const QUIC_CIPHER_ALGORITHM_AES_128: QUIC_CIPHER_ALGORITHM = QUIC_CIPHER_ALGORITHM(26126i32);
pub const QUIC_CIPHER_ALGORITHM_AES_256: QUIC_CIPHER_ALGORITHM = QUIC_CIPHER_ALGORITHM(26128i32);
pub const QUIC_CIPHER_ALGORITHM_CHACHA20: QUIC_CIPHER_ALGORITHM = QUIC_CIPHER_ALGORITHM(26130i32);
pub const QUIC_CIPHER_ALGORITHM_NONE: QUIC_CIPHER_ALGORITHM = QUIC_CIPHER_ALGORITHM(0i32);
pub const QUIC_CIPHER_SUITE_TLS_AES_128_GCM_SHA256: QUIC_CIPHER_SUITE = QUIC_CIPHER_SUITE(4865i32);
pub const QUIC_CIPHER_SUITE_TLS_AES_256_GCM_SHA384: QUIC_CIPHER_SUITE = QUIC_CIPHER_SUITE(4866i32);
pub const QUIC_CIPHER_SUITE_TLS_CHACHA20_POLY1305_SHA256: QUIC_CIPHER_SUITE =
    QUIC_CIPHER_SUITE(4867i32);
pub const QUIC_CONGESTION_CONTROL_ALGORITHM_CUBIC: QUIC_CONGESTION_CONTROL_ALGORITHM =
    QUIC_CONGESTION_CONTROL_ALGORITHM(0i32);
pub const QUIC_CONGESTION_CONTROL_ALGORITHM_MAX: QUIC_CONGESTION_CONTROL_ALGORITHM =
    QUIC_CONGESTION_CONTROL_ALGORITHM(1i32);
pub const QUIC_CONNECTION_EVENT_CONNECTED: QUIC_CONNECTION_EVENT_TYPE =
    QUIC_CONNECTION_EVENT_TYPE(0i32);
pub const QUIC_CONNECTION_EVENT_DATAGRAM_RECEIVED: QUIC_CONNECTION_EVENT_TYPE =
    QUIC_CONNECTION_EVENT_TYPE(11i32);
pub const QUIC_CONNECTION_EVENT_DATAGRAM_SEND_STATE_CHANGED: QUIC_CONNECTION_EVENT_TYPE =
    QUIC_CONNECTION_EVENT_TYPE(12i32);
pub const QUIC_CONNECTION_EVENT_DATAGRAM_STATE_CHANGED: QUIC_CONNECTION_EVENT_TYPE =
    QUIC_CONNECTION_EVENT_TYPE(10i32);
pub const QUIC_CONNECTION_EVENT_IDEAL_PROCESSOR_CHANGED: QUIC_CONNECTION_EVENT_TYPE =
    QUIC_CONNECTION_EVENT_TYPE(9i32);
pub const QUIC_CONNECTION_EVENT_LOCAL_ADDRESS_CHANGED: QUIC_CONNECTION_EVENT_TYPE =
    QUIC_CONNECTION_EVENT_TYPE(4i32);
pub const QUIC_CONNECTION_EVENT_PEER_ADDRESS_CHANGED: QUIC_CONNECTION_EVENT_TYPE =
    QUIC_CONNECTION_EVENT_TYPE(5i32);
pub const QUIC_CONNECTION_EVENT_PEER_CERTIFICATE_RECEIVED: QUIC_CONNECTION_EVENT_TYPE =
    QUIC_CONNECTION_EVENT_TYPE(15i32);
pub const QUIC_CONNECTION_EVENT_PEER_NEEDS_STREAMS: QUIC_CONNECTION_EVENT_TYPE =
    QUIC_CONNECTION_EVENT_TYPE(8i32);
pub const QUIC_CONNECTION_EVENT_PEER_STREAM_STARTED: QUIC_CONNECTION_EVENT_TYPE =
    QUIC_CONNECTION_EVENT_TYPE(6i32);
pub const QUIC_CONNECTION_EVENT_RESUMED: QUIC_CONNECTION_EVENT_TYPE =
    QUIC_CONNECTION_EVENT_TYPE(13i32);
pub const QUIC_CONNECTION_EVENT_RESUMPTION_TICKET_RECEIVED: QUIC_CONNECTION_EVENT_TYPE =
    QUIC_CONNECTION_EVENT_TYPE(14i32);
pub const QUIC_CONNECTION_EVENT_SHUTDOWN_COMPLETE: QUIC_CONNECTION_EVENT_TYPE =
    QUIC_CONNECTION_EVENT_TYPE(3i32);
pub const QUIC_CONNECTION_EVENT_SHUTDOWN_INITIATED_BY_PEER: QUIC_CONNECTION_EVENT_TYPE =
    QUIC_CONNECTION_EVENT_TYPE(2i32);
pub const QUIC_CONNECTION_EVENT_SHUTDOWN_INITIATED_BY_TRANSPORT: QUIC_CONNECTION_EVENT_TYPE =
    QUIC_CONNECTION_EVENT_TYPE(1i32);
pub const QUIC_CONNECTION_EVENT_STREAMS_AVAILABLE: QUIC_CONNECTION_EVENT_TYPE =
    QUIC_CONNECTION_EVENT_TYPE(7i32);
pub const QUIC_CONNECTION_SHUTDOWN_FLAG_NONE: QUIC_CONNECTION_SHUTDOWN_FLAGS =
    QUIC_CONNECTION_SHUTDOWN_FLAGS(0i32);
pub const QUIC_CONNECTION_SHUTDOWN_FLAG_SILENT: QUIC_CONNECTION_SHUTDOWN_FLAGS =
    QUIC_CONNECTION_SHUTDOWN_FLAGS(1i32);
pub const QUIC_CREDENTIAL_FLAG_CACHE_ONLY_URL_RETRIEVAL: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(131072i32);
pub const QUIC_CREDENTIAL_FLAG_CLIENT: QUIC_CREDENTIAL_FLAGS = QUIC_CREDENTIAL_FLAGS(1i32);
pub const QUIC_CREDENTIAL_FLAG_DEFER_CERTIFICATE_VALIDATION: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(32i32);
pub const QUIC_CREDENTIAL_FLAG_ENABLE_OCSP: QUIC_CREDENTIAL_FLAGS = QUIC_CREDENTIAL_FLAGS(8i32);
pub const QUIC_CREDENTIAL_FLAG_IGNORE_NO_REVOCATION_CHECK: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(2048i32);
pub const QUIC_CREDENTIAL_FLAG_IGNORE_REVOCATION_OFFLINE: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(4096i32);
pub const QUIC_CREDENTIAL_FLAG_INDICATE_CERTIFICATE_RECEIVED: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(16i32);
pub const QUIC_CREDENTIAL_FLAG_INPROC_PEER_CERTIFICATE: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(524288i32);
pub const QUIC_CREDENTIAL_FLAG_LOAD_ASYNCHRONOUS: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(2i32);
pub const QUIC_CREDENTIAL_FLAG_NONE: QUIC_CREDENTIAL_FLAGS = QUIC_CREDENTIAL_FLAGS(0i32);
pub const QUIC_CREDENTIAL_FLAG_NO_CERTIFICATE_VALIDATION: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(4i32);
pub const QUIC_CREDENTIAL_FLAG_REQUIRE_CLIENT_AUTHENTICATION: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(64i32);
pub const QUIC_CREDENTIAL_FLAG_REVOCATION_CHECK_CACHE_ONLY: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(262144i32);
pub const QUIC_CREDENTIAL_FLAG_REVOCATION_CHECK_CHAIN: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(512i32);
pub const QUIC_CREDENTIAL_FLAG_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(1024i32);
pub const QUIC_CREDENTIAL_FLAG_REVOCATION_CHECK_END_CERT: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(256i32);
pub const QUIC_CREDENTIAL_FLAG_SET_ALLOWED_CIPHER_SUITES: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(8192i32);
pub const QUIC_CREDENTIAL_FLAG_SET_CA_CERTIFICATE_FILE: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(1048576i32);
pub const QUIC_CREDENTIAL_FLAG_USE_PORTABLE_CERTIFICATES: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(16384i32);
pub const QUIC_CREDENTIAL_FLAG_USE_SUPPLIED_CREDENTIALS: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(32768i32);
pub const QUIC_CREDENTIAL_FLAG_USE_SYSTEM_MAPPER: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(65536i32);
pub const QUIC_CREDENTIAL_FLAG_USE_TLS_BUILTIN_CERTIFICATE_VALIDATION: QUIC_CREDENTIAL_FLAGS =
    QUIC_CREDENTIAL_FLAGS(128i32);
pub const QUIC_CREDENTIAL_TYPE_CERTIFICATE_CONTEXT: QUIC_CREDENTIAL_TYPE =
    QUIC_CREDENTIAL_TYPE(3i32);
pub const QUIC_CREDENTIAL_TYPE_CERTIFICATE_FILE: QUIC_CREDENTIAL_TYPE = QUIC_CREDENTIAL_TYPE(4i32);
pub const QUIC_CREDENTIAL_TYPE_CERTIFICATE_FILE_PROTECTED: QUIC_CREDENTIAL_TYPE =
    QUIC_CREDENTIAL_TYPE(5i32);
pub const QUIC_CREDENTIAL_TYPE_CERTIFICATE_HASH: QUIC_CREDENTIAL_TYPE = QUIC_CREDENTIAL_TYPE(1i32);
pub const QUIC_CREDENTIAL_TYPE_CERTIFICATE_HASH_STORE: QUIC_CREDENTIAL_TYPE =
    QUIC_CREDENTIAL_TYPE(2i32);
pub const QUIC_CREDENTIAL_TYPE_CERTIFICATE_PKCS12: QUIC_CREDENTIAL_TYPE =
    QUIC_CREDENTIAL_TYPE(6i32);
pub const QUIC_CREDENTIAL_TYPE_NONE: QUIC_CREDENTIAL_TYPE = QUIC_CREDENTIAL_TYPE(0i32);
pub const QUIC_DATAGRAM_SEND_ACKNOWLEDGED: QUIC_DATAGRAM_SEND_STATE =
    QUIC_DATAGRAM_SEND_STATE(4i32);
pub const QUIC_DATAGRAM_SEND_ACKNOWLEDGED_SPURIOUS: QUIC_DATAGRAM_SEND_STATE =
    QUIC_DATAGRAM_SEND_STATE(5i32);
pub const QUIC_DATAGRAM_SEND_CANCELED: QUIC_DATAGRAM_SEND_STATE = QUIC_DATAGRAM_SEND_STATE(6i32);
pub const QUIC_DATAGRAM_SEND_LOST_DISCARDED: QUIC_DATAGRAM_SEND_STATE =
    QUIC_DATAGRAM_SEND_STATE(3i32);
pub const QUIC_DATAGRAM_SEND_LOST_SUSPECT: QUIC_DATAGRAM_SEND_STATE =
    QUIC_DATAGRAM_SEND_STATE(2i32);
pub const QUIC_DATAGRAM_SEND_SENT: QUIC_DATAGRAM_SEND_STATE = QUIC_DATAGRAM_SEND_STATE(1i32);
pub const QUIC_DATAGRAM_SEND_UNKNOWN: QUIC_DATAGRAM_SEND_STATE = QUIC_DATAGRAM_SEND_STATE(0i32);
pub const QUIC_EXECUTION_CONFIG_FLAG_NONE: QUIC_EXECUTION_CONFIG_FLAGS =
    QUIC_EXECUTION_CONFIG_FLAGS(0i32);
pub const QUIC_EXECUTION_PROFILE_LOW_LATENCY: QUIC_EXECUTION_PROFILE = QUIC_EXECUTION_PROFILE(0i32);
pub const QUIC_EXECUTION_PROFILE_TYPE_MAX_THROUGHPUT: QUIC_EXECUTION_PROFILE =
    QUIC_EXECUTION_PROFILE(1i32);
pub const QUIC_EXECUTION_PROFILE_TYPE_REAL_TIME: QUIC_EXECUTION_PROFILE =
    QUIC_EXECUTION_PROFILE(3i32);
pub const QUIC_EXECUTION_PROFILE_TYPE_SCAVENGER: QUIC_EXECUTION_PROFILE =
    QUIC_EXECUTION_PROFILE(2i32);
pub const QUIC_HASH_ALGORITHM_NONE: QUIC_HASH_ALGORITHM = QUIC_HASH_ALGORITHM(0i32);
pub const QUIC_HASH_ALGORITHM_SHA_256: QUIC_HASH_ALGORITHM = QUIC_HASH_ALGORITHM(32780i32);
pub const QUIC_HASH_ALGORITHM_SHA_384: QUIC_HASH_ALGORITHM = QUIC_HASH_ALGORITHM(32781i32);
pub const QUIC_KEY_EXCHANGE_ALGORITHM_NONE: QUIC_KEY_EXCHANGE_ALGORITHM =
    QUIC_KEY_EXCHANGE_ALGORITHM(0i32);
pub const QUIC_LISTENER_EVENT_NEW_CONNECTION: QUIC_LISTENER_EVENT_TYPE =
    QUIC_LISTENER_EVENT_TYPE(0i32);
pub const QUIC_LISTENER_EVENT_STOP_COMPLETE: QUIC_LISTENER_EVENT_TYPE =
    QUIC_LISTENER_EVENT_TYPE(1i32);
pub const QUIC_LOAD_BALANCING_COUNT: QUIC_LOAD_BALANCING_MODE = QUIC_LOAD_BALANCING_MODE(3i32);
pub const QUIC_LOAD_BALANCING_DISABLED: QUIC_LOAD_BALANCING_MODE = QUIC_LOAD_BALANCING_MODE(0i32);
pub const QUIC_LOAD_BALANCING_SERVER_ID_FIXED: QUIC_LOAD_BALANCING_MODE =
    QUIC_LOAD_BALANCING_MODE(2i32);
pub const QUIC_LOAD_BALANCING_SERVER_ID_IP: QUIC_LOAD_BALANCING_MODE =
    QUIC_LOAD_BALANCING_MODE(1i32);
pub const QUIC_MAX_ALPN_LENGTH: u32 = 255u32;
pub const QUIC_MAX_RESUMPTION_APP_DATA_LENGTH: u32 = 1000u32;
pub const QUIC_MAX_SNI_LENGTH: u32 = 65535u32;
pub const QUIC_MAX_TICKET_KEY_COUNT: u32 = 16u32;
pub const QUIC_PARAM_CONFIGURATION_SCHANNEL_CREDENTIAL_ATTRIBUTE_W: u32 = 50331651u32;
pub const QUIC_PARAM_CONFIGURATION_SETTINGS: u32 = 50331648u32;
pub const QUIC_PARAM_CONFIGURATION_TICKET_KEYS: u32 = 50331649u32;
pub const QUIC_PARAM_CONFIGURATION_VERSION_SETTINGS: u32 = 50331650u32;
pub const QUIC_PARAM_CONN_CIBIR_ID: u32 = 83886101u32;
pub const QUIC_PARAM_CONN_CLOSE_REASON_PHRASE: u32 = 83886091u32;
pub const QUIC_PARAM_CONN_DATAGRAM_RECEIVE_ENABLED: u32 = 83886093u32;
pub const QUIC_PARAM_CONN_DATAGRAM_SEND_ENABLED: u32 = 83886094u32;
pub const QUIC_PARAM_CONN_DISABLE_1RTT_ENCRYPTION: u32 = 83886095u32;
pub const QUIC_PARAM_CONN_IDEAL_PROCESSOR: u32 = 83886083u32;
pub const QUIC_PARAM_CONN_LOCAL_ADDRESS: u32 = 83886081u32;
pub const QUIC_PARAM_CONN_LOCAL_BIDI_STREAM_COUNT: u32 = 83886088u32;
pub const QUIC_PARAM_CONN_LOCAL_INTERFACE: u32 = 83886098u32;
pub const QUIC_PARAM_CONN_LOCAL_UNIDI_STREAM_COUNT: u32 = 83886089u32;
pub const QUIC_PARAM_CONN_MAX_STREAM_IDS: u32 = 83886090u32;
pub const QUIC_PARAM_CONN_ORIG_DEST_CID: u32 = 83886104u32;
pub const QUIC_PARAM_CONN_PEER_CERTIFICATE_VALID: u32 = 83886097u32;
pub const QUIC_PARAM_CONN_QUIC_VERSION: u32 = 83886080u32;
pub const QUIC_PARAM_CONN_REMOTE_ADDRESS: u32 = 83886082u32;
pub const QUIC_PARAM_CONN_RESUMPTION_TICKET: u32 = 83886096u32;
pub const QUIC_PARAM_CONN_SETTINGS: u32 = 83886084u32;
pub const QUIC_PARAM_CONN_SHARE_UDP_BINDING: u32 = 83886087u32;
pub const QUIC_PARAM_CONN_STATISTICS: u32 = 83886085u32;
pub const QUIC_PARAM_CONN_STATISTICS_PLAT: u32 = 83886086u32;
pub const QUIC_PARAM_CONN_STATISTICS_V2: u32 = 83886102u32;
pub const QUIC_PARAM_CONN_STATISTICS_V2_PLAT: u32 = 83886103u32;
pub const QUIC_PARAM_CONN_STREAM_SCHEDULING_SCHEME: u32 = 83886092u32;
pub const QUIC_PARAM_CONN_TLS_SECRETS: u32 = 83886099u32;
pub const QUIC_PARAM_CONN_VERSION_SETTINGS: u32 = 83886100u32;
pub const QUIC_PARAM_GLOBAL_EXECUTION_CONFIG: u32 = 16777225u32;
pub const QUIC_PARAM_GLOBAL_GLOBAL_SETTINGS: u32 = 16777222u32;
pub const QUIC_PARAM_GLOBAL_LIBRARY_GIT_HASH: u32 = 16777224u32;
pub const QUIC_PARAM_GLOBAL_LIBRARY_VERSION: u32 = 16777220u32;
pub const QUIC_PARAM_GLOBAL_LOAD_BALACING_MODE: u32 = 16777218u32;
pub const QUIC_PARAM_GLOBAL_PERF_COUNTERS: u32 = 16777219u32;
pub const QUIC_PARAM_GLOBAL_RETRY_MEMORY_PERCENT: u32 = 16777216u32;
pub const QUIC_PARAM_GLOBAL_SETTINGS: u32 = 16777221u32;
pub const QUIC_PARAM_GLOBAL_STATELESS_RESET_KEY: u32 = 16777227u32;
pub const QUIC_PARAM_GLOBAL_SUPPORTED_VERSIONS: u32 = 16777217u32;
pub const QUIC_PARAM_GLOBAL_TLS_PROVIDER: u32 = 16777226u32;
pub const QUIC_PARAM_GLOBAL_VERSION_SETTINGS: u32 = 16777223u32;
pub const QUIC_PARAM_LISTENER_CIBIR_ID: u32 = 67108866u32;
pub const QUIC_PARAM_LISTENER_LOCAL_ADDRESS: u32 = 67108864u32;
pub const QUIC_PARAM_LISTENER_STATS: u32 = 67108865u32;
pub const QUIC_PARAM_PREFIX_CONFIGURATION: u32 = 50331648u32;
pub const QUIC_PARAM_PREFIX_CONNECTION: u32 = 83886080u32;
pub const QUIC_PARAM_PREFIX_GLOBAL: u32 = 16777216u32;
pub const QUIC_PARAM_PREFIX_LISTENER: u32 = 67108864u32;
pub const QUIC_PARAM_PREFIX_REGISTRATION: u32 = 33554432u32;
pub const QUIC_PARAM_PREFIX_STREAM: u32 = 134217728u32;
pub const QUIC_PARAM_PREFIX_TLS: u32 = 100663296u32;
pub const QUIC_PARAM_PREFIX_TLS_SCHANNEL: u32 = 117440512u32;
pub const QUIC_PARAM_STREAM_0RTT_LENGTH: u32 = 134217729u32;
pub const QUIC_PARAM_STREAM_ID: u32 = 134217728u32;
pub const QUIC_PARAM_STREAM_IDEAL_SEND_BUFFER_SIZE: u32 = 134217730u32;
pub const QUIC_PARAM_STREAM_PRIORITY: u32 = 134217731u32;
pub const QUIC_PARAM_STREAM_RELIABLE_OFFSET: u32 = 134217733u32;
pub const QUIC_PARAM_STREAM_STATISTICS: u32 = 134217732u32;
pub const QUIC_PARAM_TLS_HANDSHAKE_INFO: u32 = 100663296u32;
pub const QUIC_PARAM_TLS_NEGOTIATED_ALPN: u32 = 100663297u32;
pub const QUIC_PARAM_TLS_SCHANNEL_CONTEXT_ATTRIBUTE_EX_W: u32 = 117440513u32;
pub const QUIC_PARAM_TLS_SCHANNEL_CONTEXT_ATTRIBUTE_W: u32 = 117440512u32;
pub const QUIC_PARAM_TLS_SCHANNEL_SECURITY_CONTEXT_TOKEN: u32 = 117440514u32;
pub const QUIC_PERF_COUNTER_APP_RECV_BYTES: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(19i32);
pub const QUIC_PERF_COUNTER_APP_SEND_BYTES: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(18i32);
pub const QUIC_PERF_COUNTER_CONN_ACTIVE: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(4i32);
pub const QUIC_PERF_COUNTER_CONN_APP_REJECT: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(2i32);
pub const QUIC_PERF_COUNTER_CONN_CONNECTED: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(5i32);
pub const QUIC_PERF_COUNTER_CONN_CREATED: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(0i32);
pub const QUIC_PERF_COUNTER_CONN_HANDSHAKE_FAIL: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(1i32);
pub const QUIC_PERF_COUNTER_CONN_NO_ALPN: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(7i32);
pub const QUIC_PERF_COUNTER_CONN_OPER_COMPLETED: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(23i32);
pub const QUIC_PERF_COUNTER_CONN_OPER_QUEUED: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(22i32);
pub const QUIC_PERF_COUNTER_CONN_OPER_QUEUE_DEPTH: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(21i32);
pub const QUIC_PERF_COUNTER_CONN_PROTOCOL_ERRORS: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(6i32);
pub const QUIC_PERF_COUNTER_CONN_QUEUE_DEPTH: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(20i32);
pub const QUIC_PERF_COUNTER_CONN_RESUMED: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(3i32);
pub const QUIC_PERF_COUNTER_MAX: QUIC_PERFORMANCE_COUNTERS = QUIC_PERFORMANCE_COUNTERS(31i32);
pub const QUIC_PERF_COUNTER_PATH_FAILURE: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(28i32);
pub const QUIC_PERF_COUNTER_PATH_VALIDATED: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(27i32);
pub const QUIC_PERF_COUNTER_PKTS_DECRYPTION_FAIL: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(11i32);
pub const QUIC_PERF_COUNTER_PKTS_DROPPED: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(10i32);
pub const QUIC_PERF_COUNTER_PKTS_SUSPECTED_LOST: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(9i32);
pub const QUIC_PERF_COUNTER_SEND_STATELESS_RESET: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(29i32);
pub const QUIC_PERF_COUNTER_SEND_STATELESS_RETRY: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(30i32);
pub const QUIC_PERF_COUNTER_STRM_ACTIVE: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(8i32);
pub const QUIC_PERF_COUNTER_UDP_RECV: QUIC_PERFORMANCE_COUNTERS = QUIC_PERFORMANCE_COUNTERS(12i32);
pub const QUIC_PERF_COUNTER_UDP_RECV_BYTES: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(14i32);
pub const QUIC_PERF_COUNTER_UDP_RECV_EVENTS: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(16i32);
pub const QUIC_PERF_COUNTER_UDP_SEND: QUIC_PERFORMANCE_COUNTERS = QUIC_PERFORMANCE_COUNTERS(13i32);
pub const QUIC_PERF_COUNTER_UDP_SEND_BYTES: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(15i32);
pub const QUIC_PERF_COUNTER_UDP_SEND_CALLS: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(17i32);
pub const QUIC_PERF_COUNTER_WORK_OPER_COMPLETED: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(26i32);
pub const QUIC_PERF_COUNTER_WORK_OPER_QUEUED: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(25i32);
pub const QUIC_PERF_COUNTER_WORK_OPER_QUEUE_DEPTH: QUIC_PERFORMANCE_COUNTERS =
    QUIC_PERFORMANCE_COUNTERS(24i32);
pub const QUIC_RECEIVE_FLAG_0_RTT: QUIC_RECEIVE_FLAGS = QUIC_RECEIVE_FLAGS(1i32);
pub const QUIC_RECEIVE_FLAG_FIN: QUIC_RECEIVE_FLAGS = QUIC_RECEIVE_FLAGS(2i32);
pub const QUIC_RECEIVE_FLAG_NONE: QUIC_RECEIVE_FLAGS = QUIC_RECEIVE_FLAGS(0i32);
pub const QUIC_SEND_FLAG_ALLOW_0_RTT: QUIC_SEND_FLAGS = QUIC_SEND_FLAGS(1i32);
pub const QUIC_SEND_FLAG_CANCEL_ON_LOSS: QUIC_SEND_FLAGS = QUIC_SEND_FLAGS(32i32);
pub const QUIC_SEND_FLAG_DELAY_SEND: QUIC_SEND_FLAGS = QUIC_SEND_FLAGS(16i32);
pub const QUIC_SEND_FLAG_DGRAM_PRIORITY: QUIC_SEND_FLAGS = QUIC_SEND_FLAGS(8i32);
pub const QUIC_SEND_FLAG_FIN: QUIC_SEND_FLAGS = QUIC_SEND_FLAGS(4i32);
pub const QUIC_SEND_FLAG_NONE: QUIC_SEND_FLAGS = QUIC_SEND_FLAGS(0i32);
pub const QUIC_SEND_FLAG_START: QUIC_SEND_FLAGS = QUIC_SEND_FLAGS(2i32);
pub const QUIC_SEND_RESUMPTION_FLAG_FINAL: QUIC_SEND_RESUMPTION_FLAGS =
    QUIC_SEND_RESUMPTION_FLAGS(1i32);
pub const QUIC_SEND_RESUMPTION_FLAG_NONE: QUIC_SEND_RESUMPTION_FLAGS =
    QUIC_SEND_RESUMPTION_FLAGS(0i32);
pub const QUIC_SERVER_NO_RESUME: QUIC_SERVER_RESUMPTION_LEVEL = QUIC_SERVER_RESUMPTION_LEVEL(0i32);
pub const QUIC_SERVER_RESUME_AND_ZERORTT: QUIC_SERVER_RESUMPTION_LEVEL =
    QUIC_SERVER_RESUMPTION_LEVEL(2i32);
pub const QUIC_SERVER_RESUME_ONLY: QUIC_SERVER_RESUMPTION_LEVEL =
    QUIC_SERVER_RESUMPTION_LEVEL(1i32);
pub const QUIC_STATELESS_RESET_KEY_LENGTH: u32 = 32u32;
pub const QUIC_STREAM_EVENT_CANCEL_ON_LOSS: QUIC_STREAM_EVENT_TYPE = QUIC_STREAM_EVENT_TYPE(10i32);
pub const QUIC_STREAM_EVENT_IDEAL_SEND_BUFFER_SIZE: QUIC_STREAM_EVENT_TYPE =
    QUIC_STREAM_EVENT_TYPE(8i32);
pub const QUIC_STREAM_EVENT_PEER_ACCEPTED: QUIC_STREAM_EVENT_TYPE = QUIC_STREAM_EVENT_TYPE(9i32);
pub const QUIC_STREAM_EVENT_PEER_RECEIVE_ABORTED: QUIC_STREAM_EVENT_TYPE =
    QUIC_STREAM_EVENT_TYPE(5i32);
pub const QUIC_STREAM_EVENT_PEER_SEND_ABORTED: QUIC_STREAM_EVENT_TYPE =
    QUIC_STREAM_EVENT_TYPE(4i32);
pub const QUIC_STREAM_EVENT_PEER_SEND_SHUTDOWN: QUIC_STREAM_EVENT_TYPE =
    QUIC_STREAM_EVENT_TYPE(3i32);
pub const QUIC_STREAM_EVENT_RECEIVE: QUIC_STREAM_EVENT_TYPE = QUIC_STREAM_EVENT_TYPE(1i32);
pub const QUIC_STREAM_EVENT_SEND_COMPLETE: QUIC_STREAM_EVENT_TYPE = QUIC_STREAM_EVENT_TYPE(2i32);
pub const QUIC_STREAM_EVENT_SEND_SHUTDOWN_COMPLETE: QUIC_STREAM_EVENT_TYPE =
    QUIC_STREAM_EVENT_TYPE(6i32);
pub const QUIC_STREAM_EVENT_SHUTDOWN_COMPLETE: QUIC_STREAM_EVENT_TYPE =
    QUIC_STREAM_EVENT_TYPE(7i32);
pub const QUIC_STREAM_EVENT_START_COMPLETE: QUIC_STREAM_EVENT_TYPE = QUIC_STREAM_EVENT_TYPE(0i32);
pub const QUIC_STREAM_OPEN_FLAG_0_RTT: QUIC_STREAM_OPEN_FLAGS = QUIC_STREAM_OPEN_FLAGS(2i32);
pub const QUIC_STREAM_OPEN_FLAG_DELAY_ID_FC_UPDATES: QUIC_STREAM_OPEN_FLAGS =
    QUIC_STREAM_OPEN_FLAGS(4i32);
pub const QUIC_STREAM_OPEN_FLAG_NONE: QUIC_STREAM_OPEN_FLAGS = QUIC_STREAM_OPEN_FLAGS(0i32);
pub const QUIC_STREAM_OPEN_FLAG_UNIDIRECTIONAL: QUIC_STREAM_OPEN_FLAGS =
    QUIC_STREAM_OPEN_FLAGS(1i32);
pub const QUIC_STREAM_SCHEDULING_SCHEME_COUNT: QUIC_STREAM_SCHEDULING_SCHEME =
    QUIC_STREAM_SCHEDULING_SCHEME(2i32);
pub const QUIC_STREAM_SCHEDULING_SCHEME_FIFO: QUIC_STREAM_SCHEDULING_SCHEME =
    QUIC_STREAM_SCHEDULING_SCHEME(0i32);
pub const QUIC_STREAM_SCHEDULING_SCHEME_ROUND_ROBIN: QUIC_STREAM_SCHEDULING_SCHEME =
    QUIC_STREAM_SCHEDULING_SCHEME(1i32);
pub const QUIC_STREAM_SHUTDOWN_FLAG_ABORT: QUIC_STREAM_SHUTDOWN_FLAGS =
    QUIC_STREAM_SHUTDOWN_FLAGS(6i32);
pub const QUIC_STREAM_SHUTDOWN_FLAG_ABORT_RECEIVE: QUIC_STREAM_SHUTDOWN_FLAGS =
    QUIC_STREAM_SHUTDOWN_FLAGS(4i32);
pub const QUIC_STREAM_SHUTDOWN_FLAG_ABORT_SEND: QUIC_STREAM_SHUTDOWN_FLAGS =
    QUIC_STREAM_SHUTDOWN_FLAGS(2i32);
pub const QUIC_STREAM_SHUTDOWN_FLAG_GRACEFUL: QUIC_STREAM_SHUTDOWN_FLAGS =
    QUIC_STREAM_SHUTDOWN_FLAGS(1i32);
pub const QUIC_STREAM_SHUTDOWN_FLAG_IMMEDIATE: QUIC_STREAM_SHUTDOWN_FLAGS =
    QUIC_STREAM_SHUTDOWN_FLAGS(8i32);
pub const QUIC_STREAM_SHUTDOWN_FLAG_INLINE: QUIC_STREAM_SHUTDOWN_FLAGS =
    QUIC_STREAM_SHUTDOWN_FLAGS(16i32);
pub const QUIC_STREAM_SHUTDOWN_FLAG_NONE: QUIC_STREAM_SHUTDOWN_FLAGS =
    QUIC_STREAM_SHUTDOWN_FLAGS(0i32);
pub const QUIC_STREAM_START_FLAG_FAIL_BLOCKED: QUIC_STREAM_START_FLAGS =
    QUIC_STREAM_START_FLAGS(2i32);
pub const QUIC_STREAM_START_FLAG_IMMEDIATE: QUIC_STREAM_START_FLAGS = QUIC_STREAM_START_FLAGS(1i32);
pub const QUIC_STREAM_START_FLAG_INDICATE_PEER_ACCEPT: QUIC_STREAM_START_FLAGS =
    QUIC_STREAM_START_FLAGS(8i32);
pub const QUIC_STREAM_START_FLAG_NONE: QUIC_STREAM_START_FLAGS = QUIC_STREAM_START_FLAGS(0i32);
pub const QUIC_STREAM_START_FLAG_SHUTDOWN_ON_FAIL: QUIC_STREAM_START_FLAGS =
    QUIC_STREAM_START_FLAGS(4i32);
pub const QUIC_TLS_ALERT_CODE_ACCESS_DENIED: QUIC_TLS_ALERT_CODES = QUIC_TLS_ALERT_CODES(49i32);
pub const QUIC_TLS_ALERT_CODE_BAD_CERTIFICATE: QUIC_TLS_ALERT_CODES = QUIC_TLS_ALERT_CODES(42i32);
pub const QUIC_TLS_ALERT_CODE_CERTIFICATE_EXPIRED: QUIC_TLS_ALERT_CODES =
    QUIC_TLS_ALERT_CODES(45i32);
pub const QUIC_TLS_ALERT_CODE_CERTIFICATE_REQUIRED: QUIC_TLS_ALERT_CODES =
    QUIC_TLS_ALERT_CODES(116i32);
pub const QUIC_TLS_ALERT_CODE_CERTIFICATE_REVOKED: QUIC_TLS_ALERT_CODES =
    QUIC_TLS_ALERT_CODES(44i32);
pub const QUIC_TLS_ALERT_CODE_CERTIFICATE_UNKNOWN: QUIC_TLS_ALERT_CODES =
    QUIC_TLS_ALERT_CODES(46i32);
pub const QUIC_TLS_ALERT_CODE_ILLEGAL_PARAMETER: QUIC_TLS_ALERT_CODES = QUIC_TLS_ALERT_CODES(47i32);
pub const QUIC_TLS_ALERT_CODE_INSUFFICIENT_SECURITY: QUIC_TLS_ALERT_CODES =
    QUIC_TLS_ALERT_CODES(71i32);
pub const QUIC_TLS_ALERT_CODE_INTERNAL_ERROR: QUIC_TLS_ALERT_CODES = QUIC_TLS_ALERT_CODES(80i32);
pub const QUIC_TLS_ALERT_CODE_MAX: QUIC_TLS_ALERT_CODES = QUIC_TLS_ALERT_CODES(255i32);
pub const QUIC_TLS_ALERT_CODE_SUCCESS: QUIC_TLS_ALERT_CODES = QUIC_TLS_ALERT_CODES(65535i32);
pub const QUIC_TLS_ALERT_CODE_UNEXPECTED_MESSAGE: QUIC_TLS_ALERT_CODES =
    QUIC_TLS_ALERT_CODES(10i32);
pub const QUIC_TLS_ALERT_CODE_UNKNOWN_CA: QUIC_TLS_ALERT_CODES = QUIC_TLS_ALERT_CODES(48i32);
pub const QUIC_TLS_ALERT_CODE_UNSUPPORTED_CERTIFICATE: QUIC_TLS_ALERT_CODES =
    QUIC_TLS_ALERT_CODES(43i32);
pub const QUIC_TLS_ALERT_CODE_USER_CANCELED: QUIC_TLS_ALERT_CODES = QUIC_TLS_ALERT_CODES(90i32);
pub const QUIC_TLS_PROTOCOL_1_3: QUIC_TLS_PROTOCOL_VERSION = QUIC_TLS_PROTOCOL_VERSION(12288i32);
pub const QUIC_TLS_PROTOCOL_UNKNOWN: QUIC_TLS_PROTOCOL_VERSION = QUIC_TLS_PROTOCOL_VERSION(0i32);
pub const QUIC_TLS_PROVIDER_OPENSSL: QUIC_TLS_PROVIDER = QUIC_TLS_PROVIDER(1i32);
pub const QUIC_TLS_PROVIDER_SCHANNEL: QUIC_TLS_PROVIDER = QUIC_TLS_PROVIDER(0i32);
pub const QUIC_TLS_SECRETS_MAX_SECRET_LEN: u32 = 64u32;
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_ALLOWED_CIPHER_SUITE_FLAGS(pub i32);
impl ::windows_core::TypeKind for QUIC_ALLOWED_CIPHER_SUITE_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_ALLOWED_CIPHER_SUITE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_ALLOWED_CIPHER_SUITE_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl QUIC_ALLOWED_CIPHER_SUITE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for QUIC_ALLOWED_CIPHER_SUITE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for QUIC_ALLOWED_CIPHER_SUITE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for QUIC_ALLOWED_CIPHER_SUITE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for QUIC_ALLOWED_CIPHER_SUITE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for QUIC_ALLOWED_CIPHER_SUITE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_CERTIFICATE_HASH_STORE_FLAGS(pub i32);
impl ::windows_core::TypeKind for QUIC_CERTIFICATE_HASH_STORE_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_CERTIFICATE_HASH_STORE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_CERTIFICATE_HASH_STORE_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl QUIC_CERTIFICATE_HASH_STORE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for QUIC_CERTIFICATE_HASH_STORE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for QUIC_CERTIFICATE_HASH_STORE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for QUIC_CERTIFICATE_HASH_STORE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for QUIC_CERTIFICATE_HASH_STORE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for QUIC_CERTIFICATE_HASH_STORE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_CIPHER_ALGORITHM(pub i32);
impl ::windows_core::TypeKind for QUIC_CIPHER_ALGORITHM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_CIPHER_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_CIPHER_ALGORITHM")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_CIPHER_SUITE(pub i32);
impl ::windows_core::TypeKind for QUIC_CIPHER_SUITE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_CIPHER_SUITE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_CIPHER_SUITE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_CONGESTION_CONTROL_ALGORITHM(pub i32);
impl ::windows_core::TypeKind for QUIC_CONGESTION_CONTROL_ALGORITHM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_CONGESTION_CONTROL_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_CONGESTION_CONTROL_ALGORITHM")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_CONNECTION_EVENT_TYPE(pub i32);
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_CONNECTION_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_CONNECTION_EVENT_TYPE")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_CONNECTION_SHUTDOWN_FLAGS(pub i32);
impl ::windows_core::TypeKind for QUIC_CONNECTION_SHUTDOWN_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_CONNECTION_SHUTDOWN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_CONNECTION_SHUTDOWN_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl QUIC_CONNECTION_SHUTDOWN_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for QUIC_CONNECTION_SHUTDOWN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for QUIC_CONNECTION_SHUTDOWN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for QUIC_CONNECTION_SHUTDOWN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for QUIC_CONNECTION_SHUTDOWN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for QUIC_CONNECTION_SHUTDOWN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_CREDENTIAL_FLAGS(pub i32);
impl ::windows_core::TypeKind for QUIC_CREDENTIAL_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_CREDENTIAL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_CREDENTIAL_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl QUIC_CREDENTIAL_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for QUIC_CREDENTIAL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for QUIC_CREDENTIAL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for QUIC_CREDENTIAL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for QUIC_CREDENTIAL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for QUIC_CREDENTIAL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_CREDENTIAL_TYPE(pub i32);
impl ::windows_core::TypeKind for QUIC_CREDENTIAL_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_CREDENTIAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_CREDENTIAL_TYPE")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_DATAGRAM_SEND_STATE(pub i32);
impl ::windows_core::TypeKind for QUIC_DATAGRAM_SEND_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_DATAGRAM_SEND_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_DATAGRAM_SEND_STATE")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_EXECUTION_CONFIG_FLAGS(pub i32);
impl ::windows_core::TypeKind for QUIC_EXECUTION_CONFIG_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_EXECUTION_CONFIG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_EXECUTION_CONFIG_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl QUIC_EXECUTION_CONFIG_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for QUIC_EXECUTION_CONFIG_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for QUIC_EXECUTION_CONFIG_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for QUIC_EXECUTION_CONFIG_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for QUIC_EXECUTION_CONFIG_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for QUIC_EXECUTION_CONFIG_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_EXECUTION_PROFILE(pub i32);
impl ::windows_core::TypeKind for QUIC_EXECUTION_PROFILE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_EXECUTION_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_EXECUTION_PROFILE")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_HASH_ALGORITHM(pub i32);
impl ::windows_core::TypeKind for QUIC_HASH_ALGORITHM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_HASH_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_HASH_ALGORITHM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_KEY_EXCHANGE_ALGORITHM(pub i32);
impl ::windows_core::TypeKind for QUIC_KEY_EXCHANGE_ALGORITHM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_KEY_EXCHANGE_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_KEY_EXCHANGE_ALGORITHM")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_LISTENER_EVENT_TYPE(pub i32);
impl ::windows_core::TypeKind for QUIC_LISTENER_EVENT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_LISTENER_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_LISTENER_EVENT_TYPE")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_LOAD_BALANCING_MODE(pub i32);
impl ::windows_core::TypeKind for QUIC_LOAD_BALANCING_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_LOAD_BALANCING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_LOAD_BALANCING_MODE")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_PERFORMANCE_COUNTERS(pub i32);
impl ::windows_core::TypeKind for QUIC_PERFORMANCE_COUNTERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_PERFORMANCE_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_PERFORMANCE_COUNTERS")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_RECEIVE_FLAGS(pub i32);
impl ::windows_core::TypeKind for QUIC_RECEIVE_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_RECEIVE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_RECEIVE_FLAGS").field(&self.0).finish()
    }
}
impl QUIC_RECEIVE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for QUIC_RECEIVE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for QUIC_RECEIVE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for QUIC_RECEIVE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for QUIC_RECEIVE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for QUIC_RECEIVE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_SEND_FLAGS(pub i32);
impl ::windows_core::TypeKind for QUIC_SEND_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_SEND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_SEND_FLAGS").field(&self.0).finish()
    }
}
impl QUIC_SEND_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for QUIC_SEND_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for QUIC_SEND_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for QUIC_SEND_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for QUIC_SEND_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for QUIC_SEND_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_SEND_RESUMPTION_FLAGS(pub i32);
impl ::windows_core::TypeKind for QUIC_SEND_RESUMPTION_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_SEND_RESUMPTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_SEND_RESUMPTION_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl QUIC_SEND_RESUMPTION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for QUIC_SEND_RESUMPTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for QUIC_SEND_RESUMPTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for QUIC_SEND_RESUMPTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for QUIC_SEND_RESUMPTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for QUIC_SEND_RESUMPTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_SERVER_RESUMPTION_LEVEL(pub i32);
impl ::windows_core::TypeKind for QUIC_SERVER_RESUMPTION_LEVEL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_SERVER_RESUMPTION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_SERVER_RESUMPTION_LEVEL")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_STREAM_EVENT_TYPE(pub i32);
impl ::windows_core::TypeKind for QUIC_STREAM_EVENT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_STREAM_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_STREAM_EVENT_TYPE")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_STREAM_OPEN_FLAGS(pub i32);
impl ::windows_core::TypeKind for QUIC_STREAM_OPEN_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_STREAM_OPEN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_STREAM_OPEN_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl QUIC_STREAM_OPEN_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for QUIC_STREAM_OPEN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for QUIC_STREAM_OPEN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for QUIC_STREAM_OPEN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for QUIC_STREAM_OPEN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for QUIC_STREAM_OPEN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_STREAM_SCHEDULING_SCHEME(pub i32);
impl ::windows_core::TypeKind for QUIC_STREAM_SCHEDULING_SCHEME {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_STREAM_SCHEDULING_SCHEME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_STREAM_SCHEDULING_SCHEME")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_STREAM_SHUTDOWN_FLAGS(pub i32);
impl ::windows_core::TypeKind for QUIC_STREAM_SHUTDOWN_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_STREAM_SHUTDOWN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_STREAM_SHUTDOWN_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl QUIC_STREAM_SHUTDOWN_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for QUIC_STREAM_SHUTDOWN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for QUIC_STREAM_SHUTDOWN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for QUIC_STREAM_SHUTDOWN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for QUIC_STREAM_SHUTDOWN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for QUIC_STREAM_SHUTDOWN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_STREAM_START_FLAGS(pub i32);
impl ::windows_core::TypeKind for QUIC_STREAM_START_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_STREAM_START_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_STREAM_START_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl QUIC_STREAM_START_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for QUIC_STREAM_START_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for QUIC_STREAM_START_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for QUIC_STREAM_START_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for QUIC_STREAM_START_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for QUIC_STREAM_START_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_TLS_ALERT_CODES(pub i32);
impl ::windows_core::TypeKind for QUIC_TLS_ALERT_CODES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_TLS_ALERT_CODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_TLS_ALERT_CODES")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_TLS_PROTOCOL_VERSION(pub i32);
impl ::windows_core::TypeKind for QUIC_TLS_PROTOCOL_VERSION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_TLS_PROTOCOL_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_TLS_PROTOCOL_VERSION")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::default::Default,
)]
pub struct QUIC_TLS_PROVIDER(pub i32);
impl ::windows_core::TypeKind for QUIC_TLS_PROVIDER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUIC_TLS_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUIC_TLS_PROVIDER").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QUIC_API_TABLE {
    pub SetContext: QUIC_SET_CONTEXT_FN,
    pub GetContext: QUIC_GET_CONTEXT_FN,
    pub SetCallbackHandler: QUIC_SET_CALLBACK_HANDLER_FN,
    pub SetParam: QUIC_SET_PARAM_FN,
    pub GetParam: QUIC_GET_PARAM_FN,
    pub RegistrationOpen: QUIC_REGISTRATION_OPEN_FN,
    pub RegistrationClose: QUIC_REGISTRATION_CLOSE_FN,
    pub RegistrationShutdown: QUIC_REGISTRATION_SHUTDOWN_FN,
    pub ConfigurationOpen: QUIC_CONFIGURATION_OPEN_FN,
    pub ConfigurationClose: QUIC_CONFIGURATION_CLOSE_FN,
    pub ConfigurationLoadCredential: QUIC_CONFIGURATION_LOAD_CREDENTIAL_FN,
    pub ListenerOpen: QUIC_LISTENER_OPEN_FN,
    pub ListenerClose: QUIC_LISTENER_CLOSE_FN,
    pub ListenerStart: QUIC_LISTENER_START_FN,
    pub ListenerStop: QUIC_LISTENER_STOP_FN,
    pub ConnectionOpen: QUIC_CONNECTION_OPEN_FN,
    pub ConnectionClose: QUIC_CONNECTION_CLOSE_FN,
    pub ConnectionShutdown: QUIC_CONNECTION_SHUTDOWN_FN,
    pub ConnectionStart: QUIC_CONNECTION_START_FN,
    pub ConnectionSetConfiguration: QUIC_CONNECTION_SET_CONFIGURATION_FN,
    pub ConnectionSendResumptionTicket: QUIC_CONNECTION_SEND_RESUMPTION_FN,
    pub StreamOpen: QUIC_STREAM_OPEN_FN,
    pub StreamClose: QUIC_STREAM_CLOSE_FN,
    pub StreamStart: QUIC_STREAM_START_FN,
    pub StreamShutdown: QUIC_STREAM_SHUTDOWN_FN,
    pub StreamSend: QUIC_STREAM_SEND_FN,
    pub StreamReceiveComplete: QUIC_STREAM_RECEIVE_COMPLETE_FN,
    pub StreamReceiveSetEnabled: QUIC_STREAM_RECEIVE_SET_ENABLED_FN,
    pub DatagramSend: QUIC_DATAGRAM_SEND_FN,
    pub ConnectionResumptionTicketValidationComplete: QUIC_CONNECTION_COMP_RESUMPTION_FN,
    pub ConnectionCertificateValidationComplete: QUIC_CONNECTION_COMP_CERT_FN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_API_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_API_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QUIC_API_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_API_TABLE").finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_API_TABLE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_API_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_BUFFER {
    pub Length: u32,
    pub Buffer: *mut u8,
}
impl ::core::marker::Copy for QUIC_BUFFER {}
impl ::core::clone::Clone for QUIC_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_BUFFER")
            .field("Length", &self.Length)
            .field("Buffer", &self.Buffer)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_BUFFER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for QUIC_BUFFER {}
impl ::core::default::Default for QUIC_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_CERTIFICATE_FILE {
    pub PrivateKeyFile: ::windows_core::PCSTR,
    pub CertificateFile: ::windows_core::PCSTR,
}
impl ::core::marker::Copy for QUIC_CERTIFICATE_FILE {}
impl ::core::clone::Clone for QUIC_CERTIFICATE_FILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_CERTIFICATE_FILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CERTIFICATE_FILE")
            .field("PrivateKeyFile", &self.PrivateKeyFile)
            .field("CertificateFile", &self.CertificateFile)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_CERTIFICATE_FILE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_CERTIFICATE_FILE {
    fn eq(&self, other: &Self) -> bool {
        self.PrivateKeyFile == other.PrivateKeyFile && self.CertificateFile == other.CertificateFile
    }
}
impl ::core::cmp::Eq for QUIC_CERTIFICATE_FILE {}
impl ::core::default::Default for QUIC_CERTIFICATE_FILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_CERTIFICATE_FILE_PROTECTED {
    pub PrivateKeyFile: ::windows_core::PCSTR,
    pub CertificateFile: ::windows_core::PCSTR,
    pub PrivateKeyPassword: ::windows_core::PCSTR,
}
impl ::core::marker::Copy for QUIC_CERTIFICATE_FILE_PROTECTED {}
impl ::core::clone::Clone for QUIC_CERTIFICATE_FILE_PROTECTED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_CERTIFICATE_FILE_PROTECTED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CERTIFICATE_FILE_PROTECTED")
            .field("PrivateKeyFile", &self.PrivateKeyFile)
            .field("CertificateFile", &self.CertificateFile)
            .field("PrivateKeyPassword", &self.PrivateKeyPassword)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_CERTIFICATE_FILE_PROTECTED {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_CERTIFICATE_FILE_PROTECTED {
    fn eq(&self, other: &Self) -> bool {
        self.PrivateKeyFile == other.PrivateKeyFile
            && self.CertificateFile == other.CertificateFile
            && self.PrivateKeyPassword == other.PrivateKeyPassword
    }
}
impl ::core::cmp::Eq for QUIC_CERTIFICATE_FILE_PROTECTED {}
impl ::core::default::Default for QUIC_CERTIFICATE_FILE_PROTECTED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_CERTIFICATE_HASH {
    pub ShaHash: [u8; 20],
}
impl ::core::marker::Copy for QUIC_CERTIFICATE_HASH {}
impl ::core::clone::Clone for QUIC_CERTIFICATE_HASH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_CERTIFICATE_HASH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CERTIFICATE_HASH")
            .field("ShaHash", &self.ShaHash)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_CERTIFICATE_HASH {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_CERTIFICATE_HASH {
    fn eq(&self, other: &Self) -> bool {
        self.ShaHash == other.ShaHash
    }
}
impl ::core::cmp::Eq for QUIC_CERTIFICATE_HASH {}
impl ::core::default::Default for QUIC_CERTIFICATE_HASH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_CERTIFICATE_HASH_STORE {
    pub Flags: QUIC_CERTIFICATE_HASH_STORE_FLAGS,
    pub ShaHash: [u8; 20],
    pub StoreName: [i8; 128],
}
impl ::core::marker::Copy for QUIC_CERTIFICATE_HASH_STORE {}
impl ::core::clone::Clone for QUIC_CERTIFICATE_HASH_STORE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_CERTIFICATE_HASH_STORE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CERTIFICATE_HASH_STORE")
            .field("Flags", &self.Flags)
            .field("ShaHash", &self.ShaHash)
            .field("StoreName", &self.StoreName)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_CERTIFICATE_HASH_STORE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_CERTIFICATE_HASH_STORE {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
            && self.ShaHash == other.ShaHash
            && self.StoreName == other.StoreName
    }
}
impl ::core::cmp::Eq for QUIC_CERTIFICATE_HASH_STORE {}
impl ::core::default::Default for QUIC_CERTIFICATE_HASH_STORE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_CERTIFICATE_PKCS12 {
    pub Asn1Blob: *const u8,
    pub Asn1BlobLength: u32,
    pub PrivateKeyPassword: ::windows_core::PCSTR,
}
impl ::core::marker::Copy for QUIC_CERTIFICATE_PKCS12 {}
impl ::core::clone::Clone for QUIC_CERTIFICATE_PKCS12 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_CERTIFICATE_PKCS12 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CERTIFICATE_PKCS12")
            .field("Asn1Blob", &self.Asn1Blob)
            .field("Asn1BlobLength", &self.Asn1BlobLength)
            .field("PrivateKeyPassword", &self.PrivateKeyPassword)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_CERTIFICATE_PKCS12 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_CERTIFICATE_PKCS12 {
    fn eq(&self, other: &Self) -> bool {
        self.Asn1Blob == other.Asn1Blob
            && self.Asn1BlobLength == other.Asn1BlobLength
            && self.PrivateKeyPassword == other.PrivateKeyPassword
    }
}
impl ::core::cmp::Eq for QUIC_CERTIFICATE_PKCS12 {}
impl ::core::default::Default for QUIC_CERTIFICATE_PKCS12 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QUIC_CONNECTION_EVENT {
    pub Type: QUIC_CONNECTION_EVENT_TYPE,
    pub Anonymous: QUIC_CONNECTION_EVENT_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_CONNECTION_EVENT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_CONNECTION_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_CONNECTION_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union QUIC_CONNECTION_EVENT_0 {
    pub CONNECTED: QUIC_CONNECTION_EVENT_0_0,
    pub SHUTDOWN_INITIATED_BY_TRANSPORT: QUIC_CONNECTION_EVENT_0_14,
    pub SHUTDOWN_INITIATED_BY_PEER: QUIC_CONNECTION_EVENT_0_13,
    pub SHUTDOWN_COMPLETE: QUIC_CONNECTION_EVENT_0_12,
    pub LOCAL_ADDRESS_CHANGED: QUIC_CONNECTION_EVENT_0_5,
    pub PEER_ADDRESS_CHANGED: QUIC_CONNECTION_EVENT_0_6,
    pub PEER_STREAM_STARTED: QUIC_CONNECTION_EVENT_0_9,
    pub STREAMS_AVAILABLE: QUIC_CONNECTION_EVENT_0_15,
    pub PEER_NEEDS_STREAMS: QUIC_CONNECTION_EVENT_0_8,
    pub IDEAL_PROCESSOR_CHANGED: QUIC_CONNECTION_EVENT_0_4,
    pub DATAGRAM_STATE_CHANGED: QUIC_CONNECTION_EVENT_0_3,
    pub DATAGRAM_RECEIVED: QUIC_CONNECTION_EVENT_0_1,
    pub DATAGRAM_SEND_STATE_CHANGED: QUIC_CONNECTION_EVENT_0_2,
    pub RESUMED: QUIC_CONNECTION_EVENT_0_10,
    pub RESUMPTION_TICKET_RECEIVED: QUIC_CONNECTION_EVENT_0_11,
    pub PEER_CERTIFICATE_RECEIVED: QUIC_CONNECTION_EVENT_0_7,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_CONNECTION_EVENT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_CONNECTION_EVENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_CONNECTION_EVENT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QUIC_CONNECTION_EVENT_0_0 {
    pub SessionResumed: ::windows::Win32::Foundation::BOOLEAN,
    pub NegotiatedAlpnLength: u8,
    pub NegotiatedAlpn: *const u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_CONNECTION_EVENT_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_CONNECTION_EVENT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QUIC_CONNECTION_EVENT_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CONNECTION_EVENT_0_0")
            .field("SessionResumed", &self.SessionResumed)
            .field("NegotiatedAlpnLength", &self.NegotiatedAlpnLength)
            .field("NegotiatedAlpn", &self.NegotiatedAlpn)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for QUIC_CONNECTION_EVENT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.SessionResumed == other.SessionResumed
            && self.NegotiatedAlpnLength == other.NegotiatedAlpnLength
            && self.NegotiatedAlpn == other.NegotiatedAlpn
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for QUIC_CONNECTION_EVENT_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_CONNECTION_EVENT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QUIC_CONNECTION_EVENT_0_1 {
    pub Buffer: *const QUIC_BUFFER,
    pub Flags: QUIC_RECEIVE_FLAGS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_CONNECTION_EVENT_0_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_CONNECTION_EVENT_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QUIC_CONNECTION_EVENT_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CONNECTION_EVENT_0_1")
            .field("Buffer", &self.Buffer)
            .field("Flags", &self.Flags)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT_0_1 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for QUIC_CONNECTION_EVENT_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer && self.Flags == other.Flags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for QUIC_CONNECTION_EVENT_0_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_CONNECTION_EVENT_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QUIC_CONNECTION_EVENT_0_2 {
    pub ClientContext: *mut ::core::ffi::c_void,
    pub State: QUIC_DATAGRAM_SEND_STATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_CONNECTION_EVENT_0_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_CONNECTION_EVENT_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QUIC_CONNECTION_EVENT_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CONNECTION_EVENT_0_2")
            .field("ClientContext", &self.ClientContext)
            .field("State", &self.State)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT_0_2 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for QUIC_CONNECTION_EVENT_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.ClientContext == other.ClientContext && self.State == other.State
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for QUIC_CONNECTION_EVENT_0_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_CONNECTION_EVENT_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QUIC_CONNECTION_EVENT_0_3 {
    pub SendEnabled: ::windows::Win32::Foundation::BOOLEAN,
    pub MaxSendLength: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_CONNECTION_EVENT_0_3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_CONNECTION_EVENT_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QUIC_CONNECTION_EVENT_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CONNECTION_EVENT_0_3")
            .field("SendEnabled", &self.SendEnabled)
            .field("MaxSendLength", &self.MaxSendLength)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT_0_3 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for QUIC_CONNECTION_EVENT_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.SendEnabled == other.SendEnabled && self.MaxSendLength == other.MaxSendLength
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for QUIC_CONNECTION_EVENT_0_3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_CONNECTION_EVENT_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QUIC_CONNECTION_EVENT_0_4 {
    pub IdealProcessor: u16,
    pub PartitionIndex: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_CONNECTION_EVENT_0_4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_CONNECTION_EVENT_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QUIC_CONNECTION_EVENT_0_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CONNECTION_EVENT_0_4")
            .field("IdealProcessor", &self.IdealProcessor)
            .field("PartitionIndex", &self.PartitionIndex)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT_0_4 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for QUIC_CONNECTION_EVENT_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.IdealProcessor == other.IdealProcessor && self.PartitionIndex == other.PartitionIndex
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for QUIC_CONNECTION_EVENT_0_4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_CONNECTION_EVENT_0_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QUIC_CONNECTION_EVENT_0_5 {
    pub Address: *const ::windows::Win32::Networking::WinSock::SOCKADDR_INET,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_CONNECTION_EVENT_0_5 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_CONNECTION_EVENT_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QUIC_CONNECTION_EVENT_0_5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CONNECTION_EVENT_0_5")
            .field("Address", &self.Address)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT_0_5 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for QUIC_CONNECTION_EVENT_0_5 {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for QUIC_CONNECTION_EVENT_0_5 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_CONNECTION_EVENT_0_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QUIC_CONNECTION_EVENT_0_6 {
    pub Address: *const ::windows::Win32::Networking::WinSock::SOCKADDR_INET,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_CONNECTION_EVENT_0_6 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_CONNECTION_EVENT_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QUIC_CONNECTION_EVENT_0_6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CONNECTION_EVENT_0_6")
            .field("Address", &self.Address)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT_0_6 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for QUIC_CONNECTION_EVENT_0_6 {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for QUIC_CONNECTION_EVENT_0_6 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_CONNECTION_EVENT_0_6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QUIC_CONNECTION_EVENT_0_7 {
    pub Certificate: *mut ::core::ffi::c_void,
    pub DeferredErrorFlags: u32,
    pub DeferredStatus: ::windows_core::HRESULT,
    pub Chain: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_CONNECTION_EVENT_0_7 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_CONNECTION_EVENT_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QUIC_CONNECTION_EVENT_0_7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CONNECTION_EVENT_0_7")
            .field("Certificate", &self.Certificate)
            .field("DeferredErrorFlags", &self.DeferredErrorFlags)
            .field("DeferredStatus", &self.DeferredStatus)
            .field("Chain", &self.Chain)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT_0_7 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for QUIC_CONNECTION_EVENT_0_7 {
    fn eq(&self, other: &Self) -> bool {
        self.Certificate == other.Certificate
            && self.DeferredErrorFlags == other.DeferredErrorFlags
            && self.DeferredStatus == other.DeferredStatus
            && self.Chain == other.Chain
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for QUIC_CONNECTION_EVENT_0_7 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_CONNECTION_EVENT_0_7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QUIC_CONNECTION_EVENT_0_8 {
    pub Bidirectional: ::windows::Win32::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_CONNECTION_EVENT_0_8 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_CONNECTION_EVENT_0_8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QUIC_CONNECTION_EVENT_0_8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CONNECTION_EVENT_0_8")
            .field("Bidirectional", &self.Bidirectional)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT_0_8 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for QUIC_CONNECTION_EVENT_0_8 {
    fn eq(&self, other: &Self) -> bool {
        self.Bidirectional == other.Bidirectional
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for QUIC_CONNECTION_EVENT_0_8 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_CONNECTION_EVENT_0_8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QUIC_CONNECTION_EVENT_0_9 {
    pub Stream: *mut ::core::ffi::c_void,
    pub Flags: QUIC_STREAM_OPEN_FLAGS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_CONNECTION_EVENT_0_9 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_CONNECTION_EVENT_0_9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QUIC_CONNECTION_EVENT_0_9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CONNECTION_EVENT_0_9")
            .field("Stream", &self.Stream)
            .field("Flags", &self.Flags)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT_0_9 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for QUIC_CONNECTION_EVENT_0_9 {
    fn eq(&self, other: &Self) -> bool {
        self.Stream == other.Stream && self.Flags == other.Flags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for QUIC_CONNECTION_EVENT_0_9 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_CONNECTION_EVENT_0_9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QUIC_CONNECTION_EVENT_0_10 {
    pub ResumptionStateLength: u16,
    pub ResumptionState: *const u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_CONNECTION_EVENT_0_10 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_CONNECTION_EVENT_0_10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QUIC_CONNECTION_EVENT_0_10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CONNECTION_EVENT_0_10")
            .field("ResumptionStateLength", &self.ResumptionStateLength)
            .field("ResumptionState", &self.ResumptionState)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT_0_10 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for QUIC_CONNECTION_EVENT_0_10 {
    fn eq(&self, other: &Self) -> bool {
        self.ResumptionStateLength == other.ResumptionStateLength
            && self.ResumptionState == other.ResumptionState
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for QUIC_CONNECTION_EVENT_0_10 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_CONNECTION_EVENT_0_10 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QUIC_CONNECTION_EVENT_0_11 {
    pub ResumptionTicketLength: u32,
    pub ResumptionTicket: *const u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_CONNECTION_EVENT_0_11 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_CONNECTION_EVENT_0_11 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QUIC_CONNECTION_EVENT_0_11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CONNECTION_EVENT_0_11")
            .field("ResumptionTicketLength", &self.ResumptionTicketLength)
            .field("ResumptionTicket", &self.ResumptionTicket)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT_0_11 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for QUIC_CONNECTION_EVENT_0_11 {
    fn eq(&self, other: &Self) -> bool {
        self.ResumptionTicketLength == other.ResumptionTicketLength
            && self.ResumptionTicket == other.ResumptionTicket
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for QUIC_CONNECTION_EVENT_0_11 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_CONNECTION_EVENT_0_11 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QUIC_CONNECTION_EVENT_0_12 {
    pub _bitfield: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_CONNECTION_EVENT_0_12 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_CONNECTION_EVENT_0_12 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QUIC_CONNECTION_EVENT_0_12 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CONNECTION_EVENT_0_12")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT_0_12 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for QUIC_CONNECTION_EVENT_0_12 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for QUIC_CONNECTION_EVENT_0_12 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_CONNECTION_EVENT_0_12 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QUIC_CONNECTION_EVENT_0_13 {
    pub ErrorCode: u64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_CONNECTION_EVENT_0_13 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_CONNECTION_EVENT_0_13 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QUIC_CONNECTION_EVENT_0_13 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CONNECTION_EVENT_0_13")
            .field("ErrorCode", &self.ErrorCode)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT_0_13 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for QUIC_CONNECTION_EVENT_0_13 {
    fn eq(&self, other: &Self) -> bool {
        self.ErrorCode == other.ErrorCode
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for QUIC_CONNECTION_EVENT_0_13 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_CONNECTION_EVENT_0_13 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QUIC_CONNECTION_EVENT_0_14 {
    pub Status: ::windows_core::HRESULT,
    pub ErrorCode: u64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_CONNECTION_EVENT_0_14 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_CONNECTION_EVENT_0_14 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QUIC_CONNECTION_EVENT_0_14 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CONNECTION_EVENT_0_14")
            .field("Status", &self.Status)
            .field("ErrorCode", &self.ErrorCode)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT_0_14 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for QUIC_CONNECTION_EVENT_0_14 {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.ErrorCode == other.ErrorCode
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for QUIC_CONNECTION_EVENT_0_14 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_CONNECTION_EVENT_0_14 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QUIC_CONNECTION_EVENT_0_15 {
    pub BidirectionalCount: u16,
    pub UnidirectionalCount: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QUIC_CONNECTION_EVENT_0_15 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QUIC_CONNECTION_EVENT_0_15 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QUIC_CONNECTION_EVENT_0_15 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_CONNECTION_EVENT_0_15")
            .field("BidirectionalCount", &self.BidirectionalCount)
            .field("UnidirectionalCount", &self.UnidirectionalCount)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::TypeKind for QUIC_CONNECTION_EVENT_0_15 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for QUIC_CONNECTION_EVENT_0_15 {
    fn eq(&self, other: &Self) -> bool {
        self.BidirectionalCount == other.BidirectionalCount
            && self.UnidirectionalCount == other.UnidirectionalCount
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for QUIC_CONNECTION_EVENT_0_15 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QUIC_CONNECTION_EVENT_0_15 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_CREDENTIAL_CONFIG {
    pub Type: QUIC_CREDENTIAL_TYPE,
    pub Flags: QUIC_CREDENTIAL_FLAGS,
    pub Anonymous: QUIC_CREDENTIAL_CONFIG_0,
    pub Principal: ::windows_core::PCSTR,
    pub Reserved: *mut ::core::ffi::c_void,
    pub AsyncHandler: QUIC_CREDENTIAL_LOAD_COMPLETE_HANDLER,
    pub AllowedCipherSuites: QUIC_ALLOWED_CIPHER_SUITE_FLAGS,
    pub CaCertificateFile: ::windows_core::PCSTR,
}
impl ::core::marker::Copy for QUIC_CREDENTIAL_CONFIG {}
impl ::core::clone::Clone for QUIC_CREDENTIAL_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for QUIC_CREDENTIAL_CONFIG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for QUIC_CREDENTIAL_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union QUIC_CREDENTIAL_CONFIG_0 {
    pub CertificateHash: *mut QUIC_CERTIFICATE_HASH,
    pub CertificateHashStore: *mut QUIC_CERTIFICATE_HASH_STORE,
    pub CertificateContext: *mut ::core::ffi::c_void,
    pub CertificateFile: *mut QUIC_CERTIFICATE_FILE,
    pub CertificateFileProtected: *mut QUIC_CERTIFICATE_FILE_PROTECTED,
    pub CertificatePkcs12: *mut QUIC_CERTIFICATE_PKCS12,
}
impl ::core::marker::Copy for QUIC_CREDENTIAL_CONFIG_0 {}
impl ::core::clone::Clone for QUIC_CREDENTIAL_CONFIG_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for QUIC_CREDENTIAL_CONFIG_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for QUIC_CREDENTIAL_CONFIG_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_EXECUTION_CONFIG {
    pub Flags: QUIC_EXECUTION_CONFIG_FLAGS,
    pub PollingIdleTimeoutUs: u32,
    pub ProcessorCount: u32,
    pub ProcessorList: [u16; 1],
}
impl ::core::marker::Copy for QUIC_EXECUTION_CONFIG {}
impl ::core::clone::Clone for QUIC_EXECUTION_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_EXECUTION_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_EXECUTION_CONFIG")
            .field("Flags", &self.Flags)
            .field("PollingIdleTimeoutUs", &self.PollingIdleTimeoutUs)
            .field("ProcessorCount", &self.ProcessorCount)
            .field("ProcessorList", &self.ProcessorList)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_EXECUTION_CONFIG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_EXECUTION_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
            && self.PollingIdleTimeoutUs == other.PollingIdleTimeoutUs
            && self.ProcessorCount == other.ProcessorCount
            && self.ProcessorList == other.ProcessorList
    }
}
impl ::core::cmp::Eq for QUIC_EXECUTION_CONFIG {}
impl ::core::default::Default for QUIC_EXECUTION_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_GLOBAL_SETTINGS {
    pub Anonymous: QUIC_GLOBAL_SETTINGS_0,
    pub RetryMemoryLimit: u16,
    pub LoadBalancingMode: u16,
    pub FixedServerID: u32,
}
impl ::core::marker::Copy for QUIC_GLOBAL_SETTINGS {}
impl ::core::clone::Clone for QUIC_GLOBAL_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for QUIC_GLOBAL_SETTINGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for QUIC_GLOBAL_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union QUIC_GLOBAL_SETTINGS_0 {
    pub IsSetFlags: u64,
    pub IsSet: QUIC_GLOBAL_SETTINGS_0_0,
}
impl ::core::marker::Copy for QUIC_GLOBAL_SETTINGS_0 {}
impl ::core::clone::Clone for QUIC_GLOBAL_SETTINGS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for QUIC_GLOBAL_SETTINGS_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for QUIC_GLOBAL_SETTINGS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_GLOBAL_SETTINGS_0_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for QUIC_GLOBAL_SETTINGS_0_0 {}
impl ::core::clone::Clone for QUIC_GLOBAL_SETTINGS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_GLOBAL_SETTINGS_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_GLOBAL_SETTINGS_0_0")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_GLOBAL_SETTINGS_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_GLOBAL_SETTINGS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for QUIC_GLOBAL_SETTINGS_0_0 {}
impl ::core::default::Default for QUIC_GLOBAL_SETTINGS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_HANDSHAKE_INFO {
    pub TlsProtocolVersion: QUIC_TLS_PROTOCOL_VERSION,
    pub CipherAlgorithm: QUIC_CIPHER_ALGORITHM,
    pub CipherStrength: i32,
    pub Hash: QUIC_HASH_ALGORITHM,
    pub HashStrength: i32,
    pub KeyExchangeAlgorithm: QUIC_KEY_EXCHANGE_ALGORITHM,
    pub KeyExchangeStrength: i32,
    pub CipherSuite: QUIC_CIPHER_SUITE,
}
impl ::core::marker::Copy for QUIC_HANDSHAKE_INFO {}
impl ::core::clone::Clone for QUIC_HANDSHAKE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_HANDSHAKE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_HANDSHAKE_INFO")
            .field("TlsProtocolVersion", &self.TlsProtocolVersion)
            .field("CipherAlgorithm", &self.CipherAlgorithm)
            .field("CipherStrength", &self.CipherStrength)
            .field("Hash", &self.Hash)
            .field("HashStrength", &self.HashStrength)
            .field("KeyExchangeAlgorithm", &self.KeyExchangeAlgorithm)
            .field("KeyExchangeStrength", &self.KeyExchangeStrength)
            .field("CipherSuite", &self.CipherSuite)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_HANDSHAKE_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_HANDSHAKE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.TlsProtocolVersion == other.TlsProtocolVersion
            && self.CipherAlgorithm == other.CipherAlgorithm
            && self.CipherStrength == other.CipherStrength
            && self.Hash == other.Hash
            && self.HashStrength == other.HashStrength
            && self.KeyExchangeAlgorithm == other.KeyExchangeAlgorithm
            && self.KeyExchangeStrength == other.KeyExchangeStrength
            && self.CipherSuite == other.CipherSuite
    }
}
impl ::core::cmp::Eq for QUIC_HANDSHAKE_INFO {}
impl ::core::default::Default for QUIC_HANDSHAKE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct QUIC_LISTENER_EVENT {
    pub Type: QUIC_LISTENER_EVENT_TYPE,
    pub Anonymous: QUIC_LISTENER_EVENT_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for QUIC_LISTENER_EVENT {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for QUIC_LISTENER_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows_core::TypeKind for QUIC_LISTENER_EVENT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for QUIC_LISTENER_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union QUIC_LISTENER_EVENT_0 {
    pub NEW_CONNECTION: QUIC_LISTENER_EVENT_0_0,
    pub STOP_COMPLETE: QUIC_LISTENER_EVENT_0_1,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for QUIC_LISTENER_EVENT_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for QUIC_LISTENER_EVENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows_core::TypeKind for QUIC_LISTENER_EVENT_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for QUIC_LISTENER_EVENT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct QUIC_LISTENER_EVENT_0_0 {
    pub Info: *const QUIC_NEW_CONNECTION_INFO,
    pub Connection: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for QUIC_LISTENER_EVENT_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for QUIC_LISTENER_EVENT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for QUIC_LISTENER_EVENT_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_LISTENER_EVENT_0_0")
            .field("Info", &self.Info)
            .field("Connection", &self.Connection)
            .finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows_core::TypeKind for QUIC_LISTENER_EVENT_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for QUIC_LISTENER_EVENT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Info == other.Info && self.Connection == other.Connection
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for QUIC_LISTENER_EVENT_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for QUIC_LISTENER_EVENT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct QUIC_LISTENER_EVENT_0_1 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for QUIC_LISTENER_EVENT_0_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for QUIC_LISTENER_EVENT_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for QUIC_LISTENER_EVENT_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_LISTENER_EVENT_0_1")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows_core::TypeKind for QUIC_LISTENER_EVENT_0_1 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for QUIC_LISTENER_EVENT_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for QUIC_LISTENER_EVENT_0_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for QUIC_LISTENER_EVENT_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_LISTENER_STATISTICS {
    pub TotalAcceptedConnections: u64,
    pub TotalRejectedConnections: u64,
    pub BindingRecvDroppedPackets: u64,
}
impl ::core::marker::Copy for QUIC_LISTENER_STATISTICS {}
impl ::core::clone::Clone for QUIC_LISTENER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_LISTENER_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_LISTENER_STATISTICS")
            .field("TotalAcceptedConnections", &self.TotalAcceptedConnections)
            .field("TotalRejectedConnections", &self.TotalRejectedConnections)
            .field("BindingRecvDroppedPackets", &self.BindingRecvDroppedPackets)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_LISTENER_STATISTICS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_LISTENER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.TotalAcceptedConnections == other.TotalAcceptedConnections
            && self.TotalRejectedConnections == other.TotalRejectedConnections
            && self.BindingRecvDroppedPackets == other.BindingRecvDroppedPackets
    }
}
impl ::core::cmp::Eq for QUIC_LISTENER_STATISTICS {}
impl ::core::default::Default for QUIC_LISTENER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct QUIC_NEW_CONNECTION_INFO {
    pub QuicVersion: u32,
    pub LocalAddress: *const ::windows::Win32::Networking::WinSock::SOCKADDR_INET,
    pub RemoteAddress: *const ::windows::Win32::Networking::WinSock::SOCKADDR_INET,
    pub CryptoBufferLength: u32,
    pub ClientAlpnListLength: u16,
    pub ServerNameLength: u16,
    pub NegotiatedAlpnLength: u8,
    pub CryptoBuffer: *const u8,
    pub ClientAlpnList: *const u8,
    pub NegotiatedAlpn: *const u8,
    pub ServerName: ::windows_core::PCSTR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for QUIC_NEW_CONNECTION_INFO {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for QUIC_NEW_CONNECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for QUIC_NEW_CONNECTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_NEW_CONNECTION_INFO")
            .field("QuicVersion", &self.QuicVersion)
            .field("LocalAddress", &self.LocalAddress)
            .field("RemoteAddress", &self.RemoteAddress)
            .field("CryptoBufferLength", &self.CryptoBufferLength)
            .field("ClientAlpnListLength", &self.ClientAlpnListLength)
            .field("ServerNameLength", &self.ServerNameLength)
            .field("NegotiatedAlpnLength", &self.NegotiatedAlpnLength)
            .field("CryptoBuffer", &self.CryptoBuffer)
            .field("ClientAlpnList", &self.ClientAlpnList)
            .field("NegotiatedAlpn", &self.NegotiatedAlpn)
            .field("ServerName", &self.ServerName)
            .finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows_core::TypeKind for QUIC_NEW_CONNECTION_INFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for QUIC_NEW_CONNECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.QuicVersion == other.QuicVersion
            && self.LocalAddress == other.LocalAddress
            && self.RemoteAddress == other.RemoteAddress
            && self.CryptoBufferLength == other.CryptoBufferLength
            && self.ClientAlpnListLength == other.ClientAlpnListLength
            && self.ServerNameLength == other.ServerNameLength
            && self.NegotiatedAlpnLength == other.NegotiatedAlpnLength
            && self.CryptoBuffer == other.CryptoBuffer
            && self.ClientAlpnList == other.ClientAlpnList
            && self.NegotiatedAlpn == other.NegotiatedAlpn
            && self.ServerName == other.ServerName
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for QUIC_NEW_CONNECTION_INFO {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for QUIC_NEW_CONNECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_REGISTRATION_CONFIG {
    pub AppName: ::windows_core::PCSTR,
    pub ExecutionProfile: QUIC_EXECUTION_PROFILE,
}
impl ::core::marker::Copy for QUIC_REGISTRATION_CONFIG {}
impl ::core::clone::Clone for QUIC_REGISTRATION_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_REGISTRATION_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_REGISTRATION_CONFIG")
            .field("AppName", &self.AppName)
            .field("ExecutionProfile", &self.ExecutionProfile)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_REGISTRATION_CONFIG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_REGISTRATION_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.AppName == other.AppName && self.ExecutionProfile == other.ExecutionProfile
    }
}
impl ::core::cmp::Eq for QUIC_REGISTRATION_CONFIG {}
impl ::core::default::Default for QUIC_REGISTRATION_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_SCHANNEL_CONTEXT_ATTRIBUTE_EX_W {
    pub Attribute: u32,
    pub BufferLength: u32,
    pub Buffer: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for QUIC_SCHANNEL_CONTEXT_ATTRIBUTE_EX_W {}
impl ::core::clone::Clone for QUIC_SCHANNEL_CONTEXT_ATTRIBUTE_EX_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_SCHANNEL_CONTEXT_ATTRIBUTE_EX_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_SCHANNEL_CONTEXT_ATTRIBUTE_EX_W")
            .field("Attribute", &self.Attribute)
            .field("BufferLength", &self.BufferLength)
            .field("Buffer", &self.Buffer)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_SCHANNEL_CONTEXT_ATTRIBUTE_EX_W {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_SCHANNEL_CONTEXT_ATTRIBUTE_EX_W {
    fn eq(&self, other: &Self) -> bool {
        self.Attribute == other.Attribute
            && self.BufferLength == other.BufferLength
            && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for QUIC_SCHANNEL_CONTEXT_ATTRIBUTE_EX_W {}
impl ::core::default::Default for QUIC_SCHANNEL_CONTEXT_ATTRIBUTE_EX_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_SCHANNEL_CONTEXT_ATTRIBUTE_W {
    pub Attribute: u32,
    pub Buffer: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for QUIC_SCHANNEL_CONTEXT_ATTRIBUTE_W {}
impl ::core::clone::Clone for QUIC_SCHANNEL_CONTEXT_ATTRIBUTE_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_SCHANNEL_CONTEXT_ATTRIBUTE_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_SCHANNEL_CONTEXT_ATTRIBUTE_W")
            .field("Attribute", &self.Attribute)
            .field("Buffer", &self.Buffer)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_SCHANNEL_CONTEXT_ATTRIBUTE_W {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_SCHANNEL_CONTEXT_ATTRIBUTE_W {
    fn eq(&self, other: &Self) -> bool {
        self.Attribute == other.Attribute && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for QUIC_SCHANNEL_CONTEXT_ATTRIBUTE_W {}
impl ::core::default::Default for QUIC_SCHANNEL_CONTEXT_ATTRIBUTE_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_SCHANNEL_CREDENTIAL_ATTRIBUTE_W {
    pub Attribute: u32,
    pub BufferLength: u32,
    pub Buffer: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for QUIC_SCHANNEL_CREDENTIAL_ATTRIBUTE_W {}
impl ::core::clone::Clone for QUIC_SCHANNEL_CREDENTIAL_ATTRIBUTE_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_SCHANNEL_CREDENTIAL_ATTRIBUTE_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_SCHANNEL_CREDENTIAL_ATTRIBUTE_W")
            .field("Attribute", &self.Attribute)
            .field("BufferLength", &self.BufferLength)
            .field("Buffer", &self.Buffer)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_SCHANNEL_CREDENTIAL_ATTRIBUTE_W {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_SCHANNEL_CREDENTIAL_ATTRIBUTE_W {
    fn eq(&self, other: &Self) -> bool {
        self.Attribute == other.Attribute
            && self.BufferLength == other.BufferLength
            && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for QUIC_SCHANNEL_CREDENTIAL_ATTRIBUTE_W {}
impl ::core::default::Default for QUIC_SCHANNEL_CREDENTIAL_ATTRIBUTE_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_SETTINGS {
    pub Anonymous1: QUIC_SETTINGS_0,
    pub MaxBytesPerKey: u64,
    pub HandshakeIdleTimeoutMs: u64,
    pub IdleTimeoutMs: u64,
    pub MtuDiscoverySearchCompleteTimeoutUs: u64,
    pub TlsClientMaxSendBuffer: u32,
    pub TlsServerMaxSendBuffer: u32,
    pub StreamRecvWindowDefault: u32,
    pub StreamRecvBufferDefault: u32,
    pub ConnFlowControlWindow: u32,
    pub MaxWorkerQueueDelayUs: u32,
    pub MaxStatelessOperations: u32,
    pub InitialWindowPackets: u32,
    pub SendIdleTimeoutMs: u32,
    pub InitialRttMs: u32,
    pub MaxAckDelayMs: u32,
    pub DisconnectTimeoutMs: u32,
    pub KeepAliveIntervalMs: u32,
    pub CongestionControlAlgorithm: u16,
    pub PeerBidiStreamCount: u16,
    pub PeerUnidiStreamCount: u16,
    pub MaxBindingStatelessOperations: u16,
    pub StatelessOperationExpirationMs: u16,
    pub MinimumMtu: u16,
    pub MaximumMtu: u16,
    pub _bitfield: u8,
    pub MaxOperationsPerDrain: u8,
    pub MtuDiscoveryMissingProbeCount: u8,
    pub DestCidUpdateIdleTimeoutMs: u32,
    pub Anonymous2: QUIC_SETTINGS_1,
    pub StreamRecvWindowBidiLocalDefault: u32,
    pub StreamRecvWindowBidiRemoteDefault: u32,
    pub StreamRecvWindowUnidiDefault: u32,
}
impl ::core::marker::Copy for QUIC_SETTINGS {}
impl ::core::clone::Clone for QUIC_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for QUIC_SETTINGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for QUIC_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union QUIC_SETTINGS_0 {
    pub IsSetFlags: u64,
    pub IsSet: QUIC_SETTINGS_0_0,
}
impl ::core::marker::Copy for QUIC_SETTINGS_0 {}
impl ::core::clone::Clone for QUIC_SETTINGS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for QUIC_SETTINGS_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for QUIC_SETTINGS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_SETTINGS_0_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for QUIC_SETTINGS_0_0 {}
impl ::core::clone::Clone for QUIC_SETTINGS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_SETTINGS_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_SETTINGS_0_0")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_SETTINGS_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_SETTINGS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for QUIC_SETTINGS_0_0 {}
impl ::core::default::Default for QUIC_SETTINGS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union QUIC_SETTINGS_1 {
    pub Flags: u64,
    pub Anonymous: QUIC_SETTINGS_1_0,
}
impl ::core::marker::Copy for QUIC_SETTINGS_1 {}
impl ::core::clone::Clone for QUIC_SETTINGS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for QUIC_SETTINGS_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for QUIC_SETTINGS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_SETTINGS_1_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for QUIC_SETTINGS_1_0 {}
impl ::core::clone::Clone for QUIC_SETTINGS_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_SETTINGS_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_SETTINGS_1_0")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_SETTINGS_1_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_SETTINGS_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for QUIC_SETTINGS_1_0 {}
impl ::core::default::Default for QUIC_SETTINGS_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_STATISTICS {
    pub CorrelationId: u64,
    pub _bitfield: u32,
    pub Rtt: u32,
    pub MinRtt: u32,
    pub MaxRtt: u32,
    pub Timing: QUIC_STATISTICS_4,
    pub Handshake: QUIC_STATISTICS_0,
    pub Send: QUIC_STATISTICS_3,
    pub Recv: QUIC_STATISTICS_2,
    pub Misc: QUIC_STATISTICS_1,
}
impl ::core::marker::Copy for QUIC_STATISTICS {}
impl ::core::clone::Clone for QUIC_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_STATISTICS")
            .field("CorrelationId", &self.CorrelationId)
            .field("_bitfield", &self._bitfield)
            .field("Rtt", &self.Rtt)
            .field("MinRtt", &self.MinRtt)
            .field("MaxRtt", &self.MaxRtt)
            .field("Timing", &self.Timing)
            .field("Handshake", &self.Handshake)
            .field("Send", &self.Send)
            .field("Recv", &self.Recv)
            .field("Misc", &self.Misc)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_STATISTICS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.CorrelationId == other.CorrelationId
            && self._bitfield == other._bitfield
            && self.Rtt == other.Rtt
            && self.MinRtt == other.MinRtt
            && self.MaxRtt == other.MaxRtt
            && self.Timing == other.Timing
            && self.Handshake == other.Handshake
            && self.Send == other.Send
            && self.Recv == other.Recv
            && self.Misc == other.Misc
    }
}
impl ::core::cmp::Eq for QUIC_STATISTICS {}
impl ::core::default::Default for QUIC_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_STATISTICS_0 {
    pub ClientFlight1Bytes: u32,
    pub ServerFlight1Bytes: u32,
    pub ClientFlight2Bytes: u32,
}
impl ::core::marker::Copy for QUIC_STATISTICS_0 {}
impl ::core::clone::Clone for QUIC_STATISTICS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_STATISTICS_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_STATISTICS_0")
            .field("ClientFlight1Bytes", &self.ClientFlight1Bytes)
            .field("ServerFlight1Bytes", &self.ServerFlight1Bytes)
            .field("ClientFlight2Bytes", &self.ClientFlight2Bytes)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_STATISTICS_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_STATISTICS_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ClientFlight1Bytes == other.ClientFlight1Bytes
            && self.ServerFlight1Bytes == other.ServerFlight1Bytes
            && self.ClientFlight2Bytes == other.ClientFlight2Bytes
    }
}
impl ::core::cmp::Eq for QUIC_STATISTICS_0 {}
impl ::core::default::Default for QUIC_STATISTICS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_STATISTICS_1 {
    pub KeyUpdateCount: u32,
}
impl ::core::marker::Copy for QUIC_STATISTICS_1 {}
impl ::core::clone::Clone for QUIC_STATISTICS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_STATISTICS_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_STATISTICS_1")
            .field("KeyUpdateCount", &self.KeyUpdateCount)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_STATISTICS_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_STATISTICS_1 {
    fn eq(&self, other: &Self) -> bool {
        self.KeyUpdateCount == other.KeyUpdateCount
    }
}
impl ::core::cmp::Eq for QUIC_STATISTICS_1 {}
impl ::core::default::Default for QUIC_STATISTICS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_STATISTICS_2 {
    pub TotalPackets: u64,
    pub ReorderedPackets: u64,
    pub DroppedPackets: u64,
    pub DuplicatePackets: u64,
    pub TotalBytes: u64,
    pub TotalStreamBytes: u64,
    pub DecryptionFailures: u64,
    pub ValidAckFrames: u64,
}
impl ::core::marker::Copy for QUIC_STATISTICS_2 {}
impl ::core::clone::Clone for QUIC_STATISTICS_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_STATISTICS_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_STATISTICS_2")
            .field("TotalPackets", &self.TotalPackets)
            .field("ReorderedPackets", &self.ReorderedPackets)
            .field("DroppedPackets", &self.DroppedPackets)
            .field("DuplicatePackets", &self.DuplicatePackets)
            .field("TotalBytes", &self.TotalBytes)
            .field("TotalStreamBytes", &self.TotalStreamBytes)
            .field("DecryptionFailures", &self.DecryptionFailures)
            .field("ValidAckFrames", &self.ValidAckFrames)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_STATISTICS_2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_STATISTICS_2 {
    fn eq(&self, other: &Self) -> bool {
        self.TotalPackets == other.TotalPackets
            && self.ReorderedPackets == other.ReorderedPackets
            && self.DroppedPackets == other.DroppedPackets
            && self.DuplicatePackets == other.DuplicatePackets
            && self.TotalBytes == other.TotalBytes
            && self.TotalStreamBytes == other.TotalStreamBytes
            && self.DecryptionFailures == other.DecryptionFailures
            && self.ValidAckFrames == other.ValidAckFrames
    }
}
impl ::core::cmp::Eq for QUIC_STATISTICS_2 {}
impl ::core::default::Default for QUIC_STATISTICS_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_STATISTICS_3 {
    pub PathMtu: u16,
    pub TotalPackets: u64,
    pub RetransmittablePackets: u64,
    pub SuspectedLostPackets: u64,
    pub SpuriousLostPackets: u64,
    pub TotalBytes: u64,
    pub TotalStreamBytes: u64,
    pub CongestionCount: u32,
    pub PersistentCongestionCount: u32,
}
impl ::core::marker::Copy for QUIC_STATISTICS_3 {}
impl ::core::clone::Clone for QUIC_STATISTICS_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_STATISTICS_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_STATISTICS_3")
            .field("PathMtu", &self.PathMtu)
            .field("TotalPackets", &self.TotalPackets)
            .field("RetransmittablePackets", &self.RetransmittablePackets)
            .field("SuspectedLostPackets", &self.SuspectedLostPackets)
            .field("SpuriousLostPackets", &self.SpuriousLostPackets)
            .field("TotalBytes", &self.TotalBytes)
            .field("TotalStreamBytes", &self.TotalStreamBytes)
            .field("CongestionCount", &self.CongestionCount)
            .field("PersistentCongestionCount", &self.PersistentCongestionCount)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_STATISTICS_3 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_STATISTICS_3 {
    fn eq(&self, other: &Self) -> bool {
        self.PathMtu == other.PathMtu
            && self.TotalPackets == other.TotalPackets
            && self.RetransmittablePackets == other.RetransmittablePackets
            && self.SuspectedLostPackets == other.SuspectedLostPackets
            && self.SpuriousLostPackets == other.SpuriousLostPackets
            && self.TotalBytes == other.TotalBytes
            && self.TotalStreamBytes == other.TotalStreamBytes
            && self.CongestionCount == other.CongestionCount
            && self.PersistentCongestionCount == other.PersistentCongestionCount
    }
}
impl ::core::cmp::Eq for QUIC_STATISTICS_3 {}
impl ::core::default::Default for QUIC_STATISTICS_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_STATISTICS_4 {
    pub Start: u64,
    pub InitialFlightEnd: u64,
    pub HandshakeFlightEnd: u64,
}
impl ::core::marker::Copy for QUIC_STATISTICS_4 {}
impl ::core::clone::Clone for QUIC_STATISTICS_4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_STATISTICS_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_STATISTICS_4")
            .field("Start", &self.Start)
            .field("InitialFlightEnd", &self.InitialFlightEnd)
            .field("HandshakeFlightEnd", &self.HandshakeFlightEnd)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_STATISTICS_4 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_STATISTICS_4 {
    fn eq(&self, other: &Self) -> bool {
        self.Start == other.Start
            && self.InitialFlightEnd == other.InitialFlightEnd
            && self.HandshakeFlightEnd == other.HandshakeFlightEnd
    }
}
impl ::core::cmp::Eq for QUIC_STATISTICS_4 {}
impl ::core::default::Default for QUIC_STATISTICS_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_STATISTICS_V2 {
    pub CorrelationId: u64,
    pub _bitfield: u32,
    pub Rtt: u32,
    pub MinRtt: u32,
    pub MaxRtt: u32,
    pub TimingStart: u64,
    pub TimingInitialFlightEnd: u64,
    pub TimingHandshakeFlightEnd: u64,
    pub HandshakeClientFlight1Bytes: u32,
    pub HandshakeServerFlight1Bytes: u32,
    pub HandshakeClientFlight2Bytes: u32,
    pub SendPathMtu: u16,
    pub SendTotalPackets: u64,
    pub SendRetransmittablePackets: u64,
    pub SendSuspectedLostPackets: u64,
    pub SendSpuriousLostPackets: u64,
    pub SendTotalBytes: u64,
    pub SendTotalStreamBytes: u64,
    pub SendCongestionCount: u32,
    pub SendPersistentCongestionCount: u32,
    pub RecvTotalPackets: u64,
    pub RecvReorderedPackets: u64,
    pub RecvDroppedPackets: u64,
    pub RecvDuplicatePackets: u64,
    pub RecvTotalBytes: u64,
    pub RecvTotalStreamBytes: u64,
    pub RecvDecryptionFailures: u64,
    pub RecvValidAckFrames: u64,
    pub KeyUpdateCount: u32,
    pub SendCongestionWindow: u32,
    pub DestCidUpdateCount: u32,
    pub SendEcnCongestionCount: u32,
}
impl ::core::marker::Copy for QUIC_STATISTICS_V2 {}
impl ::core::clone::Clone for QUIC_STATISTICS_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_STATISTICS_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_STATISTICS_V2")
            .field("CorrelationId", &self.CorrelationId)
            .field("_bitfield", &self._bitfield)
            .field("Rtt", &self.Rtt)
            .field("MinRtt", &self.MinRtt)
            .field("MaxRtt", &self.MaxRtt)
            .field("TimingStart", &self.TimingStart)
            .field("TimingInitialFlightEnd", &self.TimingInitialFlightEnd)
            .field("TimingHandshakeFlightEnd", &self.TimingHandshakeFlightEnd)
            .field(
                "HandshakeClientFlight1Bytes",
                &self.HandshakeClientFlight1Bytes,
            )
            .field(
                "HandshakeServerFlight1Bytes",
                &self.HandshakeServerFlight1Bytes,
            )
            .field(
                "HandshakeClientFlight2Bytes",
                &self.HandshakeClientFlight2Bytes,
            )
            .field("SendPathMtu", &self.SendPathMtu)
            .field("SendTotalPackets", &self.SendTotalPackets)
            .field(
                "SendRetransmittablePackets",
                &self.SendRetransmittablePackets,
            )
            .field("SendSuspectedLostPackets", &self.SendSuspectedLostPackets)
            .field("SendSpuriousLostPackets", &self.SendSpuriousLostPackets)
            .field("SendTotalBytes", &self.SendTotalBytes)
            .field("SendTotalStreamBytes", &self.SendTotalStreamBytes)
            .field("SendCongestionCount", &self.SendCongestionCount)
            .field(
                "SendPersistentCongestionCount",
                &self.SendPersistentCongestionCount,
            )
            .field("RecvTotalPackets", &self.RecvTotalPackets)
            .field("RecvReorderedPackets", &self.RecvReorderedPackets)
            .field("RecvDroppedPackets", &self.RecvDroppedPackets)
            .field("RecvDuplicatePackets", &self.RecvDuplicatePackets)
            .field("RecvTotalBytes", &self.RecvTotalBytes)
            .field("RecvTotalStreamBytes", &self.RecvTotalStreamBytes)
            .field("RecvDecryptionFailures", &self.RecvDecryptionFailures)
            .field("RecvValidAckFrames", &self.RecvValidAckFrames)
            .field("KeyUpdateCount", &self.KeyUpdateCount)
            .field("SendCongestionWindow", &self.SendCongestionWindow)
            .field("DestCidUpdateCount", &self.DestCidUpdateCount)
            .field("SendEcnCongestionCount", &self.SendEcnCongestionCount)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_STATISTICS_V2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_STATISTICS_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.CorrelationId == other.CorrelationId
            && self._bitfield == other._bitfield
            && self.Rtt == other.Rtt
            && self.MinRtt == other.MinRtt
            && self.MaxRtt == other.MaxRtt
            && self.TimingStart == other.TimingStart
            && self.TimingInitialFlightEnd == other.TimingInitialFlightEnd
            && self.TimingHandshakeFlightEnd == other.TimingHandshakeFlightEnd
            && self.HandshakeClientFlight1Bytes == other.HandshakeClientFlight1Bytes
            && self.HandshakeServerFlight1Bytes == other.HandshakeServerFlight1Bytes
            && self.HandshakeClientFlight2Bytes == other.HandshakeClientFlight2Bytes
            && self.SendPathMtu == other.SendPathMtu
            && self.SendTotalPackets == other.SendTotalPackets
            && self.SendRetransmittablePackets == other.SendRetransmittablePackets
            && self.SendSuspectedLostPackets == other.SendSuspectedLostPackets
            && self.SendSpuriousLostPackets == other.SendSpuriousLostPackets
            && self.SendTotalBytes == other.SendTotalBytes
            && self.SendTotalStreamBytes == other.SendTotalStreamBytes
            && self.SendCongestionCount == other.SendCongestionCount
            && self.SendPersistentCongestionCount == other.SendPersistentCongestionCount
            && self.RecvTotalPackets == other.RecvTotalPackets
            && self.RecvReorderedPackets == other.RecvReorderedPackets
            && self.RecvDroppedPackets == other.RecvDroppedPackets
            && self.RecvDuplicatePackets == other.RecvDuplicatePackets
            && self.RecvTotalBytes == other.RecvTotalBytes
            && self.RecvTotalStreamBytes == other.RecvTotalStreamBytes
            && self.RecvDecryptionFailures == other.RecvDecryptionFailures
            && self.RecvValidAckFrames == other.RecvValidAckFrames
            && self.KeyUpdateCount == other.KeyUpdateCount
            && self.SendCongestionWindow == other.SendCongestionWindow
            && self.DestCidUpdateCount == other.DestCidUpdateCount
            && self.SendEcnCongestionCount == other.SendEcnCongestionCount
    }
}
impl ::core::cmp::Eq for QUIC_STATISTICS_V2 {}
impl ::core::default::Default for QUIC_STATISTICS_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct QUIC_STREAM_EVENT {
    pub Type: QUIC_STREAM_EVENT_TYPE,
    pub Anonymous: QUIC_STREAM_EVENT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for QUIC_STREAM_EVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for QUIC_STREAM_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for QUIC_STREAM_EVENT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for QUIC_STREAM_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union QUIC_STREAM_EVENT_0 {
    pub START_COMPLETE: QUIC_STREAM_EVENT_0_8,
    pub RECEIVE: QUIC_STREAM_EVENT_0_4,
    pub SEND_COMPLETE: QUIC_STREAM_EVENT_0_5,
    pub PEER_SEND_ABORTED: QUIC_STREAM_EVENT_0_3,
    pub PEER_RECEIVE_ABORTED: QUIC_STREAM_EVENT_0_2,
    pub SEND_SHUTDOWN_COMPLETE: QUIC_STREAM_EVENT_0_6,
    pub SHUTDOWN_COMPLETE: QUIC_STREAM_EVENT_0_7,
    pub IDEAL_SEND_BUFFER_SIZE: QUIC_STREAM_EVENT_0_1,
    pub CANCEL_ON_LOSS: QUIC_STREAM_EVENT_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for QUIC_STREAM_EVENT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for QUIC_STREAM_EVENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for QUIC_STREAM_EVENT_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for QUIC_STREAM_EVENT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct QUIC_STREAM_EVENT_0_0 {
    pub ErrorCode: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for QUIC_STREAM_EVENT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for QUIC_STREAM_EVENT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for QUIC_STREAM_EVENT_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_STREAM_EVENT_0_0")
            .field("ErrorCode", &self.ErrorCode)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for QUIC_STREAM_EVENT_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for QUIC_STREAM_EVENT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ErrorCode == other.ErrorCode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for QUIC_STREAM_EVENT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for QUIC_STREAM_EVENT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct QUIC_STREAM_EVENT_0_1 {
    pub ByteCount: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for QUIC_STREAM_EVENT_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for QUIC_STREAM_EVENT_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for QUIC_STREAM_EVENT_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_STREAM_EVENT_0_1")
            .field("ByteCount", &self.ByteCount)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for QUIC_STREAM_EVENT_0_1 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for QUIC_STREAM_EVENT_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.ByteCount == other.ByteCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for QUIC_STREAM_EVENT_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for QUIC_STREAM_EVENT_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct QUIC_STREAM_EVENT_0_2 {
    pub ErrorCode: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for QUIC_STREAM_EVENT_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for QUIC_STREAM_EVENT_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for QUIC_STREAM_EVENT_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_STREAM_EVENT_0_2")
            .field("ErrorCode", &self.ErrorCode)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for QUIC_STREAM_EVENT_0_2 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for QUIC_STREAM_EVENT_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.ErrorCode == other.ErrorCode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for QUIC_STREAM_EVENT_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for QUIC_STREAM_EVENT_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct QUIC_STREAM_EVENT_0_3 {
    pub ErrorCode: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for QUIC_STREAM_EVENT_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for QUIC_STREAM_EVENT_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for QUIC_STREAM_EVENT_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_STREAM_EVENT_0_3")
            .field("ErrorCode", &self.ErrorCode)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for QUIC_STREAM_EVENT_0_3 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for QUIC_STREAM_EVENT_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.ErrorCode == other.ErrorCode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for QUIC_STREAM_EVENT_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for QUIC_STREAM_EVENT_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct QUIC_STREAM_EVENT_0_4 {
    pub AbsoluteOffset: u64,
    pub TotalBufferLength: u64,
    pub Buffers: *const QUIC_BUFFER,
    pub BufferCount: u32,
    pub Flags: QUIC_RECEIVE_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for QUIC_STREAM_EVENT_0_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for QUIC_STREAM_EVENT_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for QUIC_STREAM_EVENT_0_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_STREAM_EVENT_0_4")
            .field("AbsoluteOffset", &self.AbsoluteOffset)
            .field("TotalBufferLength", &self.TotalBufferLength)
            .field("Buffers", &self.Buffers)
            .field("BufferCount", &self.BufferCount)
            .field("Flags", &self.Flags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for QUIC_STREAM_EVENT_0_4 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for QUIC_STREAM_EVENT_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.AbsoluteOffset == other.AbsoluteOffset
            && self.TotalBufferLength == other.TotalBufferLength
            && self.Buffers == other.Buffers
            && self.BufferCount == other.BufferCount
            && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for QUIC_STREAM_EVENT_0_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for QUIC_STREAM_EVENT_0_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct QUIC_STREAM_EVENT_0_5 {
    pub Canceled: ::windows::Win32::Foundation::BOOLEAN,
    pub ClientContext: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for QUIC_STREAM_EVENT_0_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for QUIC_STREAM_EVENT_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for QUIC_STREAM_EVENT_0_5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_STREAM_EVENT_0_5")
            .field("Canceled", &self.Canceled)
            .field("ClientContext", &self.ClientContext)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for QUIC_STREAM_EVENT_0_5 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for QUIC_STREAM_EVENT_0_5 {
    fn eq(&self, other: &Self) -> bool {
        self.Canceled == other.Canceled && self.ClientContext == other.ClientContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for QUIC_STREAM_EVENT_0_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for QUIC_STREAM_EVENT_0_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct QUIC_STREAM_EVENT_0_6 {
    pub Graceful: ::windows::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for QUIC_STREAM_EVENT_0_6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for QUIC_STREAM_EVENT_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for QUIC_STREAM_EVENT_0_6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_STREAM_EVENT_0_6")
            .field("Graceful", &self.Graceful)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for QUIC_STREAM_EVENT_0_6 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for QUIC_STREAM_EVENT_0_6 {
    fn eq(&self, other: &Self) -> bool {
        self.Graceful == other.Graceful
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for QUIC_STREAM_EVENT_0_6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for QUIC_STREAM_EVENT_0_6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct QUIC_STREAM_EVENT_0_7 {
    pub ConnectionShutdown: ::windows::Win32::Foundation::BOOLEAN,
    pub _bitfield: u8,
    pub ConnectionErrorCode: u64,
    pub ConnectionCloseStatus: ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for QUIC_STREAM_EVENT_0_7 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for QUIC_STREAM_EVENT_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for QUIC_STREAM_EVENT_0_7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_STREAM_EVENT_0_7")
            .field("ConnectionShutdown", &self.ConnectionShutdown)
            .field("_bitfield", &self._bitfield)
            .field("ConnectionErrorCode", &self.ConnectionErrorCode)
            .field("ConnectionCloseStatus", &self.ConnectionCloseStatus)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for QUIC_STREAM_EVENT_0_7 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for QUIC_STREAM_EVENT_0_7 {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectionShutdown == other.ConnectionShutdown
            && self._bitfield == other._bitfield
            && self.ConnectionErrorCode == other.ConnectionErrorCode
            && self.ConnectionCloseStatus == other.ConnectionCloseStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for QUIC_STREAM_EVENT_0_7 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for QUIC_STREAM_EVENT_0_7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct QUIC_STREAM_EVENT_0_8 {
    pub Status: ::windows_core::HRESULT,
    pub ID: u64,
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for QUIC_STREAM_EVENT_0_8 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for QUIC_STREAM_EVENT_0_8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for QUIC_STREAM_EVENT_0_8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_STREAM_EVENT_0_8")
            .field("Status", &self.Status)
            .field("ID", &self.ID)
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for QUIC_STREAM_EVENT_0_8 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for QUIC_STREAM_EVENT_0_8 {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.ID == other.ID && self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for QUIC_STREAM_EVENT_0_8 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for QUIC_STREAM_EVENT_0_8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_STREAM_STATISTICS {
    pub ConnBlockedBySchedulingUs: u64,
    pub ConnBlockedByPacingUs: u64,
    pub ConnBlockedByAmplificationProtUs: u64,
    pub ConnBlockedByCongestionControlUs: u64,
    pub ConnBlockedByFlowControlUs: u64,
    pub StreamBlockedByIdFlowControlUs: u64,
    pub StreamBlockedByFlowControlUs: u64,
    pub StreamBlockedByAppUs: u64,
}
impl ::core::marker::Copy for QUIC_STREAM_STATISTICS {}
impl ::core::clone::Clone for QUIC_STREAM_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_STREAM_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_STREAM_STATISTICS")
            .field("ConnBlockedBySchedulingUs", &self.ConnBlockedBySchedulingUs)
            .field("ConnBlockedByPacingUs", &self.ConnBlockedByPacingUs)
            .field(
                "ConnBlockedByAmplificationProtUs",
                &self.ConnBlockedByAmplificationProtUs,
            )
            .field(
                "ConnBlockedByCongestionControlUs",
                &self.ConnBlockedByCongestionControlUs,
            )
            .field(
                "ConnBlockedByFlowControlUs",
                &self.ConnBlockedByFlowControlUs,
            )
            .field(
                "StreamBlockedByIdFlowControlUs",
                &self.StreamBlockedByIdFlowControlUs,
            )
            .field(
                "StreamBlockedByFlowControlUs",
                &self.StreamBlockedByFlowControlUs,
            )
            .field("StreamBlockedByAppUs", &self.StreamBlockedByAppUs)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_STREAM_STATISTICS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_STREAM_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.ConnBlockedBySchedulingUs == other.ConnBlockedBySchedulingUs
            && self.ConnBlockedByPacingUs == other.ConnBlockedByPacingUs
            && self.ConnBlockedByAmplificationProtUs == other.ConnBlockedByAmplificationProtUs
            && self.ConnBlockedByCongestionControlUs == other.ConnBlockedByCongestionControlUs
            && self.ConnBlockedByFlowControlUs == other.ConnBlockedByFlowControlUs
            && self.StreamBlockedByIdFlowControlUs == other.StreamBlockedByIdFlowControlUs
            && self.StreamBlockedByFlowControlUs == other.StreamBlockedByFlowControlUs
            && self.StreamBlockedByAppUs == other.StreamBlockedByAppUs
    }
}
impl ::core::cmp::Eq for QUIC_STREAM_STATISTICS {}
impl ::core::default::Default for QUIC_STREAM_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_TICKET_KEY_CONFIG {
    pub Id: [u8; 16],
    pub Material: [u8; 64],
    pub MaterialLength: u8,
}
impl ::core::marker::Copy for QUIC_TICKET_KEY_CONFIG {}
impl ::core::clone::Clone for QUIC_TICKET_KEY_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_TICKET_KEY_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_TICKET_KEY_CONFIG")
            .field("Id", &self.Id)
            .field("Material", &self.Material)
            .field("MaterialLength", &self.MaterialLength)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_TICKET_KEY_CONFIG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_TICKET_KEY_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id
            && self.Material == other.Material
            && self.MaterialLength == other.MaterialLength
    }
}
impl ::core::cmp::Eq for QUIC_TICKET_KEY_CONFIG {}
impl ::core::default::Default for QUIC_TICKET_KEY_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_TLS_SECRETS {
    pub SecretLength: u8,
    pub IsSet: QUIC_TLS_SECRETS_0,
    pub ClientRandom: [u8; 32],
    pub ClientEarlyTrafficSecret: [u8; 64],
    pub ClientHandshakeTrafficSecret: [u8; 64],
    pub ServerHandshakeTrafficSecret: [u8; 64],
    pub ClientTrafficSecret0: [u8; 64],
    pub ServerTrafficSecret0: [u8; 64],
}
impl ::core::marker::Copy for QUIC_TLS_SECRETS {}
impl ::core::clone::Clone for QUIC_TLS_SECRETS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_TLS_SECRETS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_TLS_SECRETS")
            .field("SecretLength", &self.SecretLength)
            .field("IsSet", &self.IsSet)
            .field("ClientRandom", &self.ClientRandom)
            .field("ClientEarlyTrafficSecret", &self.ClientEarlyTrafficSecret)
            .field(
                "ClientHandshakeTrafficSecret",
                &self.ClientHandshakeTrafficSecret,
            )
            .field(
                "ServerHandshakeTrafficSecret",
                &self.ServerHandshakeTrafficSecret,
            )
            .field("ClientTrafficSecret0", &self.ClientTrafficSecret0)
            .field("ServerTrafficSecret0", &self.ServerTrafficSecret0)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_TLS_SECRETS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_TLS_SECRETS {
    fn eq(&self, other: &Self) -> bool {
        self.SecretLength == other.SecretLength
            && self.IsSet == other.IsSet
            && self.ClientRandom == other.ClientRandom
            && self.ClientEarlyTrafficSecret == other.ClientEarlyTrafficSecret
            && self.ClientHandshakeTrafficSecret == other.ClientHandshakeTrafficSecret
            && self.ServerHandshakeTrafficSecret == other.ServerHandshakeTrafficSecret
            && self.ClientTrafficSecret0 == other.ClientTrafficSecret0
            && self.ServerTrafficSecret0 == other.ServerTrafficSecret0
    }
}
impl ::core::cmp::Eq for QUIC_TLS_SECRETS {}
impl ::core::default::Default for QUIC_TLS_SECRETS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct QUIC_TLS_SECRETS_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for QUIC_TLS_SECRETS_0 {}
impl ::core::clone::Clone for QUIC_TLS_SECRETS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUIC_TLS_SECRETS_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUIC_TLS_SECRETS_0")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::windows_core::TypeKind for QUIC_TLS_SECRETS_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for QUIC_TLS_SECRETS_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for QUIC_TLS_SECRETS_0 {}
impl ::core::default::Default for QUIC_TLS_SECRETS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type MsQuicCloseFn =
    ::core::option::Option<unsafe extern "system" fn(quicapi: *const ::core::ffi::c_void)>;
pub type MsQuicOpenVersionFn = ::core::option::Option<
    unsafe extern "system" fn(
        version: u32,
        quicapi: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
>;
pub type QUIC_CONFIGURATION_CLOSE_FN =
    ::core::option::Option<unsafe extern "system" fn(configuration: *const ::core::ffi::c_void)>;
pub type QUIC_CONFIGURATION_LOAD_CREDENTIAL_FN = ::core::option::Option<
    unsafe extern "system" fn(
        configuration: *const ::core::ffi::c_void,
        credconfig: *const QUIC_CREDENTIAL_CONFIG,
    ) -> ::windows_core::HRESULT,
>;
pub type QUIC_CONFIGURATION_OPEN_FN = ::core::option::Option<
    unsafe extern "system" fn(
        registration: *const ::core::ffi::c_void,
        alpnbuffers: *const QUIC_BUFFER,
        alpnbuffercount: u32,
        settings: *const QUIC_SETTINGS,
        settingssize: u32,
        context: *const ::core::ffi::c_void,
        configuration: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type QUIC_CONNECTION_CALLBACK = ::core::option::Option<
    unsafe extern "system" fn(
        connection: *const ::core::ffi::c_void,
        context: *const ::core::ffi::c_void,
        event: *mut QUIC_CONNECTION_EVENT,
    ) -> ::windows_core::HRESULT,
>;
pub type QUIC_CONNECTION_CALLBACK_HANDLER =
    ::core::option::Option<unsafe extern "system" fn() -> ::windows_core::HRESULT>;
pub type QUIC_CONNECTION_CLOSE_FN =
    ::core::option::Option<unsafe extern "system" fn(connection: *const ::core::ffi::c_void)>;
#[cfg(feature = "Win32_Foundation")]
pub type QUIC_CONNECTION_COMP_CERT_FN = ::core::option::Option<
    unsafe extern "system" fn(
        connection: *const ::core::ffi::c_void,
        result: ::windows::Win32::Foundation::BOOLEAN,
        tlsalert: QUIC_TLS_ALERT_CODES,
    ) -> ::windows_core::HRESULT,
>;
#[cfg(feature = "Win32_Foundation")]
pub type QUIC_CONNECTION_COMP_RESUMPTION_FN = ::core::option::Option<
    unsafe extern "system" fn(
        connection: *const ::core::ffi::c_void,
        result: ::windows::Win32::Foundation::BOOLEAN,
    ) -> ::windows_core::HRESULT,
>;
pub type QUIC_CONNECTION_OPEN_FN = ::core::option::Option<
    unsafe extern "system" fn(
        registration: *const ::core::ffi::c_void,
        handler: QUIC_CONNECTION_CALLBACK_HANDLER,
        context: *const ::core::ffi::c_void,
        connection: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
>;
pub type QUIC_CONNECTION_SEND_RESUMPTION_FN = ::core::option::Option<
    unsafe extern "system" fn(
        connection: *const ::core::ffi::c_void,
        flags: QUIC_SEND_RESUMPTION_FLAGS,
        datalength: u16,
        resumptiondata: *const u8,
    ) -> ::windows_core::HRESULT,
>;
pub type QUIC_CONNECTION_SET_CONFIGURATION_FN = ::core::option::Option<
    unsafe extern "system" fn(
        connection: *const ::core::ffi::c_void,
        configuration: *const ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
>;
pub type QUIC_CONNECTION_SHUTDOWN_FN = ::core::option::Option<
    unsafe extern "system" fn(
        connection: *const ::core::ffi::c_void,
        flags: QUIC_CONNECTION_SHUTDOWN_FLAGS,
        errorcode: u64,
    ),
>;
pub type QUIC_CONNECTION_START_FN = ::core::option::Option<
    unsafe extern "system" fn(
        connection: *const ::core::ffi::c_void,
        configuration: *const ::core::ffi::c_void,
        family: u16,
        servername: ::windows_core::PCSTR,
        serverport: u16,
    ) -> ::windows_core::HRESULT,
>;
pub type QUIC_CREDENTIAL_LOAD_COMPLETE = ::core::option::Option<
    unsafe extern "system" fn(
        configuration: *const ::core::ffi::c_void,
        context: *const ::core::ffi::c_void,
        status: ::windows_core::HRESULT,
    ),
>;
pub type QUIC_CREDENTIAL_LOAD_COMPLETE_HANDLER =
    ::core::option::Option<unsafe extern "system" fn()>;
pub type QUIC_DATAGRAM_SEND_FN = ::core::option::Option<
    unsafe extern "system" fn(
        connection: *const ::core::ffi::c_void,
        buffers: *const QUIC_BUFFER,
        buffercount: u32,
        flags: QUIC_SEND_FLAGS,
        clientsendcontext: *const ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
>;
pub type QUIC_GET_CONTEXT_FN = ::core::option::Option<
    unsafe extern "system" fn(handle: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
>;
pub type QUIC_GET_PARAM_FN = ::core::option::Option<
    unsafe extern "system" fn(
        handle: *const ::core::ffi::c_void,
        param: u32,
        bufferlength: *mut u32,
        buffer: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
>;
#[cfg(feature = "Win32_Networking_WinSock")]
pub type QUIC_LISTENER_CALLBACK = ::core::option::Option<
    unsafe extern "system" fn(
        listener: *const ::core::ffi::c_void,
        context: *const ::core::ffi::c_void,
        event: *mut QUIC_LISTENER_EVENT,
    ) -> ::windows_core::HRESULT,
>;
pub type QUIC_LISTENER_CALLBACK_HANDLER =
    ::core::option::Option<unsafe extern "system" fn() -> ::windows_core::HRESULT>;
pub type QUIC_LISTENER_CLOSE_FN =
    ::core::option::Option<unsafe extern "system" fn(listener: *const ::core::ffi::c_void)>;
pub type QUIC_LISTENER_OPEN_FN = ::core::option::Option<
    unsafe extern "system" fn(
        registration: *const ::core::ffi::c_void,
        handler: QUIC_LISTENER_CALLBACK_HANDLER,
        context: *const ::core::ffi::c_void,
        listener: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
>;
#[cfg(feature = "Win32_Networking_WinSock")]
pub type QUIC_LISTENER_START_FN = ::core::option::Option<
    unsafe extern "system" fn(
        listener: *const ::core::ffi::c_void,
        alpnbuffers: *const QUIC_BUFFER,
        alpnbuffercount: u32,
        localaddress: *const ::windows::Win32::Networking::WinSock::SOCKADDR_INET,
    ) -> ::windows_core::HRESULT,
>;
pub type QUIC_LISTENER_STOP_FN =
    ::core::option::Option<unsafe extern "system" fn(listener: *const ::core::ffi::c_void)>;
pub type QUIC_REGISTRATION_CLOSE_FN =
    ::core::option::Option<unsafe extern "system" fn(registration: *const ::core::ffi::c_void)>;
pub type QUIC_REGISTRATION_OPEN_FN = ::core::option::Option<
    unsafe extern "system" fn(
        config: *const QUIC_REGISTRATION_CONFIG,
        registration: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
>;
pub type QUIC_REGISTRATION_SHUTDOWN_FN = ::core::option::Option<
    unsafe extern "system" fn(
        registration: *const ::core::ffi::c_void,
        flags: QUIC_CONNECTION_SHUTDOWN_FLAGS,
        errorcode: u64,
    ),
>;
pub type QUIC_SET_CALLBACK_HANDLER_FN = ::core::option::Option<
    unsafe extern "system" fn(
        handle: *const ::core::ffi::c_void,
        handler: *const ::core::ffi::c_void,
        context: *const ::core::ffi::c_void,
    ),
>;
pub type QUIC_SET_CONTEXT_FN = ::core::option::Option<
    unsafe extern "system" fn(
        handle: *const ::core::ffi::c_void,
        context: *const ::core::ffi::c_void,
    ),
>;
pub type QUIC_SET_PARAM_FN = ::core::option::Option<
    unsafe extern "system" fn(
        handle: *const ::core::ffi::c_void,
        param: u32,
        bufferlength: u32,
        buffer: *const ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
>;
#[cfg(feature = "Win32_Foundation")]
pub type QUIC_STREAM_CALLBACK = ::core::option::Option<
    unsafe extern "system" fn(
        stream: *const ::core::ffi::c_void,
        context: *const ::core::ffi::c_void,
        event: *mut QUIC_STREAM_EVENT,
    ) -> ::windows_core::HRESULT,
>;
pub type QUIC_STREAM_CALLBACK_HANDLER =
    ::core::option::Option<unsafe extern "system" fn() -> ::windows_core::HRESULT>;
pub type QUIC_STREAM_CLOSE_FN =
    ::core::option::Option<unsafe extern "system" fn(stream: *const ::core::ffi::c_void)>;
pub type QUIC_STREAM_OPEN_FN = ::core::option::Option<
    unsafe extern "system" fn(
        connection: *const ::core::ffi::c_void,
        flags: QUIC_STREAM_OPEN_FLAGS,
        handler: QUIC_STREAM_CALLBACK_HANDLER,
        context: *const ::core::ffi::c_void,
        stream: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
>;
pub type QUIC_STREAM_RECEIVE_COMPLETE_FN = ::core::option::Option<
    unsafe extern "system" fn(stream: *const ::core::ffi::c_void, bufferlength: u64),
>;
#[cfg(feature = "Win32_Foundation")]
pub type QUIC_STREAM_RECEIVE_SET_ENABLED_FN = ::core::option::Option<
    unsafe extern "system" fn(
        stream: *const ::core::ffi::c_void,
        isenabled: ::windows::Win32::Foundation::BOOLEAN,
    ) -> ::windows_core::HRESULT,
>;
pub type QUIC_STREAM_SEND_FN = ::core::option::Option<
    unsafe extern "system" fn(
        stream: *const ::core::ffi::c_void,
        buffers: *const QUIC_BUFFER,
        buffercount: u32,
        flags: QUIC_SEND_FLAGS,
        clientsendcontext: *const ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
>;
pub type QUIC_STREAM_SHUTDOWN_FN = ::core::option::Option<
    unsafe extern "system" fn(
        stream: *const ::core::ffi::c_void,
        flags: QUIC_STREAM_SHUTDOWN_FLAGS,
        errorcode: u64,
    ) -> ::windows_core::HRESULT,
>;
pub type QUIC_STREAM_START_FN = ::core::option::Option<
    unsafe extern "system" fn(
        stream: *const ::core::ffi::c_void,
        flags: QUIC_STREAM_START_FLAGS,
    ) -> ::windows_core::HRESULT,
>;
