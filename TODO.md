* [x] 404 | 500 - endpoint
    * [x] 404 | 500 endpoints tests
* [x] change password - endpoint
* [ ] OAuth - github
    * [ ] Migration user (if from OAuth - disable change password)
* [x] Add footnote - Only Private usage
* [x] Add favicon.ico
* [x] Add to toml optimization flags
* [ ] Fix tracing with actix-web (json format + delete logging to file)
    * [ ] Add 'save to file' to docker compose
* [x] Test app in docker using github action
    * [x] Publish only if end successful
    * [x] Publish target runtime not tests
* [x] Redis use pool as a session and as a web::Data (deadpool-redis)
    * [ ] Change timeout of deadpool-redis
    * [x] Add Redis to health_check
* [ ] Redis error handling
* [x] Change 'expect' to e500().context() for endpoints
* [x] Add to health_check html page (rendered using string)
* [ ] Change error handling to enum error
* [ ] Use mail api (sendgrid) to register
