# Authentication API

## Login [GET /api/libre/v1/login]

### Description
Redirects the user to the Casdoor login page.

### Response
- **200 OK**: Returns the URL for the Casdoor login page.

### Example
```json
{
  "url": "https://example.com/login?redirect_uri=http://localhost:8082/callback"
}
```
## Signup [GET /api/libre/v1/signup]

### Description
Redirects the user to the Casdoor signup page.

### Response
- **200 OK**: Returns the URL for the Casdoor signup page.

### Example
```json
{
  "url": "https://example.com/signup?redirect_uri=http://localhost:8082/callback"
}
```

## Callback [GET /api/libre/v1/auth/{code}]

### Description
Handles the callback from Casdoor after user authentication. Exchanges the authorization code for a token and retrieves user information.

### Parameters
- **code**: The authorization code returned by Casdoor.

### Response
- **200 OK**: Returns the authenticated user information.
- **500 Internal Server Error**: If there is an error during token exchange or user information retrieval.

## Logout [GET /api/libre/v1/logout]

### Description
Logs the user out and redirects to the home page.

### Response
- **302 Found**: Redirects to the home page.