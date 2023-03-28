# AVA Finance Alert REST API

## POST /add_alert_request
This API adds a new alert request to the database.

**Request Body**
The request body should contain a JSON object with the following fields:

- `user_id` (string, required): The user ID for the alert request.
- `asset_id` (string, required): The asset ID for the alert request.
- `above` (boolean, required): Whether the alert should be triggered when the price goes above (true) or below (false) the specified price.
- `price_alert` (string, required): The price at which the alert should be triggered.

Example:

```json
{
    "user_id": "abc123",
    "asset_id": "BTCUSDT",
    "above": true,
    "price_alert": "50000"
}
```

**Response**
If the request is successful, the response will be a JSON object with a single field:

- `id` (integer): The ID of the newly created alert request.

If any required field is missing or the price_alert is not a valid number, a 400 Bad Request response will be returned.

## POST /delete_alert_request

This API deletes an existing alert request from the database.

**Request Body**

The request body should contain a JSON object with the following field:

`id` (integer, required): The ID of the alert request to delete.
Example:

```json
{
    "id": 1
}
```

**Response**

If the alert request is successfully deleted, a 200 OK response will be returned. If the specified ID does not exist, a 404 Not Found response will be returned. If the id field is missing or invalid, a 400 Bad Request response will be returned.

## POST /update_alert_request
This API updates an existing alert request in the database.

**Request Body**
The request body should contain a JSON object with the following fields:

- `id` (integer, required): The ID of the alert request to update.
- `user_id` (string, required): The new user ID for the alert request.
- `asset_id` (string, required): The new asset ID for the alert request.
- `above` (boolean, required): Whether the alert should be triggered when the price goes above (true) or below (false) the specified price.
- `price_alert` (string, required): The new price at which the alert should be triggered.

Example:

```
{
    "id": 1,
    "user_id": "new_user_id",
    "asset_id": "ETHUSDT",
    "above": false,
    "price_alert": "1000"
}
```

**Response**

If the alert request is successfully updated, a 200 OK response will be returned. If the specified ID does not exist, a 404 Not Found response will be returned. If any required field is missing or the price_alert is not a valid number, a 400 Bad Request response will be returned.

## POST /fetch_user_alert_requests

This API fetches all alert requests associated with a given user ID from the database.

**Request Body**
The request body should contain a JSON object with the following fields:

`user_id` (string, required): The ID of the user whose alert requests are to be fetched.
Example:
```json
{
    "user_id": "example_user_id"
}
```

**Response**

If the specified user has alert requests in the database, a 200 OK response will be returned with a JSON array of alert request objects. Each alert request object will contain the following fields:

- `id` (integer): The ID of the alert request.
- `user_id` (string): The ID of the user associated with the alert request.
- `asset_id` (string): The ID of the asset associated with the alert request.
- `above` (boolean): Whether the alert should be triggered when the price goes above (true) or below (false) the specified price.
- `price_alert` (string): The price at which the alert should be triggered.
notified (boolean): Whether the user has been notified about the alert (true) or not (false).

Example:
```json
{
    "id": 1,
    "user_id": "example_user_id",
    "asset_id": "BTCUSDT",
    "above": true,
    "price_alert": "50000",
    "notified": true
},
{
    "id": 2,
    "user_id": "example_user_id",
    "asset_id": "ETHUSDT",
    "above": false,
    "price_alert": "2000",
    "notified": false
}
```

## GET /fetch_alert_requests
This API retrieves all alert requests stored in the database.

**Request Body**
This API does not require a request body.

**Response**
A 200 OK response will be returned with a JSON array containing all alert requests in the following format:

```json
[
    {
        "id": 1,
        "user_id": "user_id_1",
        "asset_id": "BTC",
        "above": true,
        "price_alert": "50000",
        "notified": false
    },
    {
        "id": 2,
        "user_id": "user_id_2",
        "asset_id": "ETH",
        "above": false,
        "price_alert": "2000",
        "notified": true
    },
    ...
]
```

If no alert requests are found in the database, an empty array will be returned.

## GET /fetch_asset_prices

This API fetches the latest prices of all assets stored in the database.

**Request Body**
This API does not require a request body.

**Response**

The response body will contain a JSON object with an array of asset prices, where each asset price has the following fields:

- `id` (integer): The ID of the asset price record.
- `asset_id` (string): The ID of the asset for which the price is being fetched.
- `price` (string): The latest price of the asset.

Example Response:

```json
[
    {
        "id": 1,
        "asset_id": "BTC",
        "price": "55000.00"
    },
    {
        "id": 2,
        "asset_id": "ETH",
        "price": "2000.00"
    }
]
```