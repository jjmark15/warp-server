use warp::Filter;

use crate::ports::http::warp::handlers::admin;

pub(crate) fn admin_filters(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("admin").and(status())
}

fn status() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("status")
        .and(warp::path::end())
        .and(warp::get())
        .map(admin::status)
}
