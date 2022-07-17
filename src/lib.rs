use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use log::info;
use serde::{Deserialize, Serialize};

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(HttpConfigHeaderRoot {
            field_name: String::new()
        })
    });
}}

struct HttpConfigHeader {
    field_name: String
}

impl Context for HttpConfigHeader {}

impl HttpContext for HttpConfigHeader {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        info!("on_http_request_headers");        
        //if let Some(value) = self.get_http_request_header("x-custom-auth") {
        //    if self.secret == value {
        //        info!("on_http_request_headers allowing");
        //        return Action::Continue;
        //    }
        //}
        //info!("on_http_request_headers blocking");
        //self.send_http_response(401, Vec::new(), None);
        //Action::Pause   
        Action::Continue   
    }

    fn on_http_request_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        info!("on_http_request_body");
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        info!("on_http_response_headers");
        Action::Continue
    }

    fn on_http_response_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        info!("on_http_response_body");
        Action::Continue
    }
}

#[derive(Serialize, Deserialize)]
struct PolicyConfig {
     #[serde(alias = "field-name")]
    field_name: String
}

struct HttpConfigHeaderRoot {
    field_name: String
}

impl Context for HttpConfigHeaderRoot {}

impl RootContext for HttpConfigHeaderRoot {
    fn on_configure(&mut self, _: usize) -> bool {
        if let Some(config_bytes) = self.get_plugin_configuration() {
            let config:PolicyConfig = serde_json::from_slice(config_bytes.as_slice()).unwrap();
            self.field_name = config.field_name;
            info!("field name is {}",self.field_name);
        }
        true
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HttpConfigHeader {
            field_name: self.field_name.clone(),
        }))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

