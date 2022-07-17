use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{Value};

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
        if !_end_of_stream {
            // Wait -- we'll be called again when the complete body is buffered
            // at the host side.
            info!("on_http_response_body wait end of stream");
            return Action::Pause;
        }

        // Replace the attribute masking it.
        // Since we returned "Pause" previuously, this will return the whole body.
        if let Some(body_bytes) = self.get_http_response_body(0, _body_size) {
            info!("on_http_response_body wait read body");
            let body_str = String::from_utf8(body_bytes).unwrap();
            let body_str_new = transform (body_str,String::from(self.field_name.as_mut()));
            self.set_http_response_body(0, _body_size, &body_str_new.into_bytes());            
        }
        Action::Continue
    } 
}

fn transform (input: String, field: String) -> String {
   info!("transform function");    
   let mut v: Value = serde_json::from_str(input.as_str()).unwrap();
   if let Some(_field_value) = v.get(field.as_str()) {
       info!("transform function field found");    
       v[field] = serde_json::Value::String("############".to_owned());
   }
   return v.to_string();
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

