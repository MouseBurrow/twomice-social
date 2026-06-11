use easy_errors::define_errors;

define_errors!(
    SocialError {
        UniqueViolation => {
            code: "23505",
            status: CONFLICT,
            message: "Friend request already exists or already friends"
        },
        FriendRequestNotFound => {
            code: "S0001",
            status: NOT_FOUND,
            message: "Friend request not found"
        },
        AlreadyFriends => {
            code: "S0002",
            status: CONFLICT,
            message: "Already friends"
        },
        CannotFriendSelf => {
            code: "S0003",
            status: BAD_REQUEST,
            message: "Cannot send friend request to yourself"
        },
        Unauthorized => {
            code: "S0004",
            status: UNAUTHORIZED,
            message: "Not authorized to perform this action"
        },
        DatabaseError => {
            code: "S0005",
            status: INTERNAL_SERVER_ERROR,
            message: "Database error"
        }
    }
);
