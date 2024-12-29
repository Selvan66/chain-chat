# ChainChat
E2E chat


TODO:

* [ ] 404 | 500 - endpoint
* [ ] change password - endpoint
* [ ] OAuth - github
* [x] Add footnote - Only Private usage
* [x] Add favicon.ico
* [ ] Add to toml optimization flags
* [ ] Fix tracing with actix-web (json format + delete logging to file)
    * [ ] Add 'save to file' to docker compose
* [x] Test app in docker using github action
    * [x] Publish only if end successful
    * [x] Publish target runtime not tests
* [ ] Redis use pool as a session and as a web::Data
* [ ] Redis error handling
* [ ] Change 'expect' to e500().context() for endpoints
* [ ] Add to health_check html page (rendered using string)
* [ ] 404 | 500 endpoints tests
