LOOKING FOR:
* [ ] parse JSON request body
* [ ] return JSON response body
* [ ] JSON request logging
* [X] JSON logger passed through contexts

    using slog, slog_json, and slog_async to pass an arc'd async slog via
    web::Data

* [ ] prometheus/openmetrics request metrics
* [ ] prometheus/openmetrics passed through contexts
* [ ] opentracing request tracing
* [ ] opentracing passed through contexts
* [ ] spec-based unit tests
* [ ] acceptance test framework
* [ ] graceful shutdown
