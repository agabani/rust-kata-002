use crate::observability::metrics;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::middleware::Logger;
use actix_web::{middleware, Error};
use regex::RegexSet;
use std::collections::HashSet;
use std::future::{Future, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

const METRICS_EXCLUDE: &str = "/metrics";
const HEALTH_EXCLUDE_REGEX: &str = "^/health(?:/.*)?$";

pub fn logger_middleware() -> Logger {
    middleware::Logger::default()
        .exclude(METRICS_EXCLUDE)
        .exclude_regex(HEALTH_EXCLUDE_REGEX)
}

pub fn metric_middleware() -> ObservabilityMetrics {
    ObservabilityMetrics::default()
        .exclude(METRICS_EXCLUDE)
        .exclude_regex(HEALTH_EXCLUDE_REGEX)
}

#[derive(Clone)]
pub struct ObservabilityMetrics {
    exclude: HashSet<String>,
    exclude_regex: RegexSet,
}

impl ObservabilityMetrics {
    pub fn new() -> Self {
        ObservabilityMetrics {
            exclude: HashSet::new(),
            exclude_regex: RegexSet::empty(),
        }
    }

    pub fn exclude<T: Into<String>>(mut self, path: T) -> Self {
        self.exclude.insert(path.into());
        self
    }

    pub fn exclude_regex<T: Into<String>>(mut self, path: T) -> Self {
        let mut patterns = self.exclude_regex.patterns().to_vec();
        patterns.push(path.into());
        self.exclude_regex = RegexSet::new(patterns).unwrap();
        self
    }
}

impl Default for ObservabilityMetrics {
    fn default() -> Self {
        ObservabilityMetrics {
            exclude: HashSet::new(),
            exclude_regex: RegexSet::empty(),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ObservabilityMetrics
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ObservabilityMetricsMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(ObservabilityMetricsMiddleware {
            service,
            options: self.clone(),
        }))
    }
}

pub struct ObservabilityMetricsMiddleware<S> {
    options: ObservabilityMetrics,
    service: S,
}

impl<S, B> Service<ServiceRequest> for ObservabilityMetricsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: ServiceRequest) -> Self::Future {
        let path = request.path().to_owned();

        if self.options.exclude.contains(&path) || self.options.exclude_regex.is_match(&path) {
            Box::pin(self.service.call(request))
        } else {
            let request_start = Instant::now();

            metrics::http_request_count(&path);

            let future = self.service.call(request);

            Box::pin(async move {
                let response = future.await? as ServiceResponse<B>;

                metrics::http_response_count(&path, &response.status());
                metrics::http_response_duration_seconds(&path, &response.status())
                    .observe(request_start.elapsed().as_secs_f64());

                Ok(response)
            })
        }
    }
}
