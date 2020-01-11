extern crate headless_chrome;

use headless_chrome::{
    Browser, Tab,
    browser::LaunchOptionsBuilder
};

use std::{{env, fs}};
use std::{thread, time};
use std::sync::Arc;

pub struct Router {
    router_ip: String,
    router_login: String,
    router_password: String
}

impl Router {

    pub fn new() -> Router {
        let router_ip = env::var("ROUTER_IP").expect("Specify ROUTER_IP environment variable");
        let router_login = env::var("ROUTER_LOGIN").expect("Specify ROUTER_LOGIN environment variable");
        let router_password = env::var("ROUTER_PASSWORD").expect("Specify ROUTER_PASSWORD environment variable");
        Router {
            router_ip: router_ip,
            router_login: router_login,
            router_password: router_password
        }
    }

    pub fn vpn_status(&self) -> Result<String, failure::Error> {
        let (_browser, tab) = self.login()?;

        tab.navigate_to(&format!("http://{}/api/vpn/l2tp_settings", self.router_ip))?;
        tab.wait_until_navigated()?;

        let vpn_status_node_children = tab.wait_for_element("connection_status")?
            .get_description()?.children
            .unwrap();
        let vpn_status_node_first_child = &vpn_status_node_children[0];

        let vpn_status = vpn_status_node_first_child.node_value.clone();
        println!("VPN status: {:?}", vpn_status);
        Ok(vpn_status)
    }

    pub fn vpn_activate(&self) {
        self.vpn_activation(true).expect("Can't activate VPN");
    }

    pub fn vpn_deactivate(&self) {
        self.vpn_activation(false).expect("Can't deactivate VPN");
    }

    fn init_browser(&self) -> (Browser, Arc<Tab>) {
        println!("Initialising browser session...");
        // initialize browser
        let browser_options = LaunchOptionsBuilder::default()
            .sandbox(false)
            .build()
            .unwrap();
        let browser = Browser::new(browser_options).unwrap();
        let tab = browser.wait_for_initial_tab().unwrap();
        (browser, tab)
    }

    fn login(&self) -> Result<(Browser, Arc<Tab>), failure::Error> { 
        // init browser
        let (browser, tab) = self.init_browser();
        
        // navigate to router homepage
        tab.navigate_to(&format!("http://{}/html/quicksetup.html", self.router_ip))?;
    
        // log in
        tab.wait_for_element("input#username")?.type_into(&self.router_login.to_owned())?;
        tab.wait_for_element("input#password")?.type_into(&self.router_password.to_owned())?;
        tab.press_key("Enter")?;
        thread::sleep(time::Duration::from_millis(2000));
        if cfg!(debug_assertions) {
            let pdf_data = tab.print_to_pdf(None)?;
            fs::write("screenshot0.pdf", &pdf_data)?;
        }
        Ok((browser, tab))
    }

    fn vpn_activation(&self, activate: bool) -> Result<(), failure::Error> {
        let (_browser, tab) = self.login()?;

        // navigate to VPN settings page 
        tab.navigate_to(&format!("http://{}/html/vpnsettings.html", self.router_ip))?;
        tab.wait_until_navigated()?;
    
        // test current checkbox value
        let checked: bool = tab.wait_for_element("input#l2tp_enable")?
            .call_js_fn("function() { return this.checked }", false)?
            .value
            .expect("Could not figure out if the VPN is actived or not!")
            .as_bool().unwrap();
        
        // enable/disable VPN connection if needed
        if checked && !activate || !checked && activate {
            if activate {
                println!("Initiating VPN connection...");
            } else {
                println!("Shutting down VPN connection...");
            }
    
            // change the checkbox status
            tab.wait_for_element("input#l2tp_enable")?.click()?;
            tab.wait_for_element("input#apply_button")?.click()?;
            thread::sleep(time::Duration::from_millis(2000));

            if cfg!(debug_assertions) {
                let pdf_data = tab.print_to_pdf(None)?;
                fs::write("screenshot.pdf", &pdf_data)?;
            }
        }
    
        Ok(())
    }

}