use axum::response::Response;
use tracing::info;

pub async fn mw_response_map(res: Response) -> Response {
	info!("->> {:<12} - mw_response_map\n", "RES_MAPPER");
	res
}
