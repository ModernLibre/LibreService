# User Management API

## Get User Count [GET /api/libre/v1/users/count/{is_online}]

Get count of users with online status filter.

+ Parameters
    + is_online: `true` (string, required) - Filter by online status

+ Response 200 (application/json)
    + Body

## Get User [GET /api/libre/v1/users/{name}]

Get details of a user by name.

+ Parameters
    + name: `john_doe` (string, required) - Name of the user

+ Response 200 (application/json)
    + Body

## Get User List [GET /api/libre/v1/users/list]

Get list of all users.

+ Response 200 (application/json)
    + Body

## Delete User [POST /api/libre/v1/users/delete]

Delete a user.

+ Request (application/json)
    + Body
        + user: (CasdoorUser, required) - User object to be deleted

+ Response 200 (application/json)
    + Body

## Add User [POST /api/libre/v1/users/add]

Add a new user.

+ Request (application/json)
    + Body
        + user: (CasdoorUser, required) - User object to be added

+ Response 200 (application/json)
    + Body