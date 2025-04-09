use candid::{CandidType, Principal};
use serde::{Serialize, Deserialize};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static NODE_REGISTRY: RefCell<HashMap<Principal, Node>> = RefCell::new(HashMap::new());
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
struct Node {
    name: String,
    principal: Principal,
    multiaddress: String,
    last_heartbeat: u64, // Timestamp in seconds
}

#[derive(CandidType, Serialize, Deserialize)]
struct RegisterResponse {
    success: bool,
    principal: Principal,
}

#[derive(CandidType, Serialize, Deserialize)]
struct VerificationRequest {
    message: Vec<u8>,
    requesting_principal: Principal,
}

#[derive(CandidType, Serialize, Deserialize)]
struct VerificationResponse {
    signature: Vec<u8>,
    signer_principal: Principal,
}

#[ic_cdk::update]
fn register_node(name: String, multiaddr: String, node_principal: Principal) -> RegisterResponse {
    NODE_REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let timestamp = ic_cdk::api::time() / 1_000_000_000; // Convert nanoseconds to seconds
        let node = Node { 
            name, 
            principal: node_principal, 
            multiaddress: multiaddr, 
            last_heartbeat: timestamp 
        };
        registry.insert(node_principal, node);
        RegisterResponse {
            success: true,
            principal: node_principal,
        }
    })
}

#[ic_cdk::update]
fn heartbeat(node_principal: Principal, name: String, multiaddr: String) -> bool {
    NODE_REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let timestamp = ic_cdk::api::time() / 1_000_000_000;
        if let Some(node) = registry.get_mut(&node_principal) {
            node.last_heartbeat = timestamp;
            if node.name != name {
                node.name = name; // Update name if changed
            }
            if node.multiaddress != multiaddr {
                node.multiaddress = multiaddr; // Update multiaddress if changed
            }
            true
        } else {
            false // Node not registered
        }
    })
}

#[ic_cdk::query]
fn get_nodes() -> Vec<Node> {
    NODE_REGISTRY.with(|registry| {
        registry.borrow().values().cloned().collect()
    })
}

#[ic_cdk::update]
fn request_signature(request: VerificationRequest) -> VerificationResponse {
    let caller = ic_cdk::caller();
    ic_cdk::println!("Received signature request from: {}", caller.to_text());

    // Verify that the requesting principal matches the caller
    if request.requesting_principal != caller {
        ic_cdk::trap("Requesting principal does not match caller");
    }

    // TODO: In a production environment, you would:
    // 1. Validate the message format
    // 2. Use a proper signing key
    // 3. Implement rate limiting
    // 4. Add additional security checks

    // For now, we'll just sign the message with a dummy signature
    let signature = request.message.clone(); // Placeholder: just return the message as the signature

    VerificationResponse {
        signature,
        signer_principal: caller,
    }
}