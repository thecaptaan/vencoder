use crate::models::response::SuccessResponse;

pub fn hello_index()->SuccessResponse{
    SuccessResponse{
        success: true,
        message: "Good Health".into(),
    }
}