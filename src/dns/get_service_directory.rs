// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

use {helper, ParameterPacket, ResponseType, Action};
use safe_dns::dns_operations::DnsOperations;

#[derive(RustcDecodable, Debug)]
pub struct GetServiceDirectory {
    pub long_name: String,
    pub service_name: String,
}

impl Action for GetServiceDirectory {
    fn execute(&mut self, params: ParameterPacket) -> ResponseType {
        let dns_operations = try!(DnsOperations::new(params.client.clone()));
        let directory_key = try!(dns_operations.get_service_home_directory_key(&self.long_name,
                                                                               &self.service_name,
                                                                               None));
        let response = try!(helper::get_dir_response(params.client.clone(), directory_key));
        Ok(Some(try!(::rustc_serialize::json::encode(&response))))
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use dns::add_service::AddService;
    use dns::register_dns::RegisterDns;
    use Action;
    use test_utils;
    use safe_core::utility;
    use safe_nfs::helper::directory_helper::DirectoryHelper;
    use safe_nfs::{AccessLevel, UNVERSIONED_DIRECTORY_LISTING_TAG};

    const TEST_DIR_NAME: &'static str = "test_dir";

    #[test]
    fn add_dns_service() {
        let parameter_packet = unwrap_result!(test_utils::get_parameter_packet(false));

        let dir_helper = DirectoryHelper::new(parameter_packet.client.clone());
        let mut app_root_dir = unwrap_result!(dir_helper.get(&parameter_packet.app_root_dir_key));
        let _ = unwrap_result!(dir_helper.create(TEST_DIR_NAME.to_string(),
                                                 UNVERSIONED_DIRECTORY_LISTING_TAG,
                                                 Vec::new(),
                                                 false,
                                                 AccessLevel::Public,
                                                 Some(&mut app_root_dir)));
        let public_name = unwrap_result!(utility::generate_random_string(10));
        let mut register_request = RegisterDns {
            long_name: public_name.clone(),
            service_name: "www".to_string(),
            is_path_shared: false,
            service_home_dir_path: format!("/{}", TEST_DIR_NAME).to_string(),
        };
        assert!(register_request.execute(parameter_packet.clone()).is_ok());

        let mut request = AddService {
            long_name: public_name.clone(),
            service_name: "blog".to_string(),
            is_path_shared: false,
            service_home_dir_path: format!("/{}", TEST_DIR_NAME).to_string(),
        };

        assert!(request.execute(parameter_packet.clone()).is_ok());

        let mut get_service_directory_request = GetServiceDirectory {
            long_name: public_name,
            service_name: "www".to_string(),
        };
        let response = get_service_directory_request.execute(parameter_packet);
        assert!(response.is_ok());
        let response_json = unwrap_result!(response);
        assert!(response_json.is_some());
    }
}
