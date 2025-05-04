#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use deadpool_postgres::Pool;
    
    use crate::controller::kyc_controller;
    use crate::models::kyc_models::{CreateKycRequest, KycVerificationRequest};

    fn create_test_pool() -> Pool {
        todo!("Implement test database setup")
    }

    #[actix_web::test]
    async fn test_create_kyc() {
        let pool = create_test_pool();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(kyc_controller::config),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/kyc/create")
            .set_json(CreateKycRequest {
                user_id: 1,
                full_name: "Test User".to_string(),
                date_of_birth: "01-01-1990".to_string(),
                id_type: "passport".to_string(),
                id_number: "AB123456".to_string(),
                address: "123 Test St, Test City".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_verify_kyc() {
        let pool = create_test_pool();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(kyc_controller::config),
        )
        .await;

        // First create a KYC record
        let create_req = test::TestRequest::post()
            .uri("/api/kyc/create")
            .set_json(CreateKycRequest {
                user_id: 1,
                full_name: "Test User".to_string(),
                date_of_birth: "01-01-1990".to_string(),
                id_type: "passport".to_string(),
                id_number: "AB123456".to_string(),
                address: "123 Test St, Test City".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, create_req).await;
        assert!(resp.status().is_success());

        // Extract the KYC ID from the response
        let kyc_response: serde_json::Value = test::read_body_json(resp).await;
        let kyc_id = kyc_response["id"].as_i64().unwrap();

        // Now verify the KYC
        let verify_req = test::TestRequest::post()
            .uri("/api/kyc/verify")
            .set_json(KycVerificationRequest {
                id: kyc_id,
                verification_status: "verified".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, verify_req).await;
        assert!(resp.status().is_success());

        let verification_response: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(
            verification_response["verification_status"].as_str().unwrap(),
            "verified"
        );
    }

    #[actix_web::test]
    async fn test_get_kyc_by_user() {
        let pool = create_test_pool();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(kyc_controller::config),
        )
        .await;

        // First create a KYC record
        let create_req = test::TestRequest::post()
            .uri("/api/kyc/create")
            .set_json(CreateKycRequest {
                user_id: 1,
                full_name: "Test User".to_string(),
                date_of_birth: "01-01-1990".to_string(),
                id_type: "passport".to_string(),
                id_number: "AB123456".to_string(),
                address: "123 Test St, Test City".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, create_req).await;
        assert!(resp.status().is_success());

        // Now get KYC by user ID
        let get_req = test::TestRequest::get()
            .uri("/api/kyc/user?user_id=1")
            .to_request();

        let resp = test::call_service(&app, get_req).await;
        assert!(resp.status().is_success());

        let kyc_response: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(kyc_response["user_id"].as_i64().unwrap(), 1);
    }

    #[actix_web::test]
    async fn test_withdrawal_with_unverified_kyc() {
        let pool = create_test_pool();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(crate::controller::withdrawal_history_controller::config),
        )
        .await;

        // Create a withdrawal request for a user with unverified KYC
        let withdrawal_req = test::TestRequest::post()
            .uri("/api/withdrawals/record")
            .set_json(serde_json::json!({
                "wallet_id": 1, // User with unverified KYC
                "plan_id": 1,
                "amount": 100.0,
                "payer_name": "Test User"
            }))
            .to_request();

        let resp = test::call_service(&app, withdrawal_req).await;
        
        // Should be forbidden (403) since KYC is not verified
        assert_eq!(resp.status().as_u16(), 403);

        let error_response: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(
            error_response["error"].as_str().unwrap(),
            "KYC verification required"
        );
    }

    #[actix_web::test]
    async fn test_withdrawal_with_verified_kyc() {
        let pool = create_test_pool();
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(kyc_controller::config)
                .configure(crate::controller::withdrawal_history_controller::config),
        )
        .await;

        // First create a KYC record
        let create_req = test::TestRequest::post()
            .uri("/api/kyc/create")
            .set_json(CreateKycRequest {
                user_id: 1,
                full_name: "Test User".to_string(),
                date_of_birth: "01-01-1990".to_string(),
                id_type: "passport".to_string(),
                id_number: "AB123456".to_string(),
                address: "123 Test St, Test City".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, create_req).await;
        assert!(resp.status().is_success());

        let kyc_response: serde_json::Value = test::read_body_json(resp).await;
        let kyc_id = kyc_response["id"].as_i64().unwrap();

        // Now verify the KYC
        let verify_req = test::TestRequest::post()
            .uri("/api/kyc/verify")
            .set_json(KycVerificationRequest {
                id: kyc_id,
                verification_status: "verified".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, verify_req).await;
        assert!(resp.status().is_success());

        // Now try to make a withdrawal with verified KYC
        let withdrawal_req = test::TestRequest::post()
            .uri("/api/withdrawals/record")
            .set_json(serde_json::json!({
                "wallet_id": 1, // User with verified KYC
                "plan_id": 1,
                "amount": 100.0,
                "payer_name": "Test User"
            }))
            .to_request();

        let resp = test::call_service(&app, withdrawal_req).await;
        
        // Should be created (201) since KYC is verified
        assert_eq!(resp.status().as_u16(), 201);
    }
} 