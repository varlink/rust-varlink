#![doc(
    html_logo_url = "https://varlink.org/images/varlink.png",
    html_favicon_url = "https://varlink.org/images/varlink-small.png"
)]
#![allow(unused_imports)]

pub mod org_varlink_resolver;
pub mod org_varlink_service;

#[cfg(test)]
mod tests {
    use static_assertions::assert_impl_all;

    #[test]
    fn org_varlink_resolver_error_is_sync_send() {
        assert_impl_all!(crate::org_varlink_resolver::Error: Send, Sync);
    }

    #[test]
    fn org_varlink_service_error_is_sync_send() {
        assert_impl_all!(crate::org_varlink_service::Error: Send, Sync);
    }
}
