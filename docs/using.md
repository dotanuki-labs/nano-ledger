# Using nano-ledger

## Creating accounts

> [!INFO]
> POST /accounts/new
>

- Example request to create an Account with no funds

```bash
curl 'http://127.0.0.1:3000/accounts/new' \
    -X POST \
    -H 'Content-Type: application/json; charset=utf-8' \
    --data-raw '{
      "alias": "ufs.main"
    }'
```

- Example request to create an Account with a pre-defined balance (in cents)

```bash
curl 'http://127.0.0.1:3000/accounts/new' \
    -X POST \
    -H 'Content-Type: application/json; charset=utf-8' \
    --data-raw '{
      "alias": "external.visa",
      "balance": 1000000
    }'
```

Example response:

```text
HTTP/1.1 200 OK
content-type: application/json
content-length: 84
date: Thu, 05 Jun 2025 19:07:43 GMT

{
  "account_id": "2a3613b7-e155-44c6-8d6d-2e758697c763",
  "alias": "ufs.main",
  "balance": 0
}
```

## Fetching account details

> [!INFO]
> GET /accounts/{account_id}
>

Example request to fetch details about an Account:

```bash
curl 'http://127.0.0.1:3000/accounts/4f543247-8160-4951-8bce-baf8e927025c'
```

Example response:

```text
HTTP/1.1 200 OK
content-type: application/json
content-length: 96
date: Fri, 06 Jun 2025 11:36:21 GMT

{
  "account_id": "4f543247-8160-4951-8bce-baf8e927025c",
  "alias": "external.visa",
  "balance": 34598000
}
```

## Creating a transaction

> [!INFO]
> POST /transactions/new
>

Example request to create a left-sided credit transaction

```bash
curl 'http://127.0.0.1:3000/transactions/new' \
    -X POST \
    -H 'Content-Type: application/json; charset=utf-8' \
    --data-raw '{
      "movement_type": "Credit",
      "lhs_account_id": "f06c7f2d-2a21-466e-a5e6-bd40b37580a4",
      "rhs_account_id": "4f543247-8160-4951-8bce-baf8e927025c",
      "description": "SEPA Transfer",
      "amount_in_cents": 10000
    }'
```

Example response

```text
HTTP/1.1 200 OK
content-type: application/json
content-length: 100
date: Fri, 06 Jun 2025 11:40:16 GMT

{
  "created_at": "2025-06-06T11:40:16.589983Z",
  "transaction_id": "cfdd279d-f174-4c99-8d83-7b059e24fd25"
}
```

## Fetching transaction details

> [!INFO]
> GET /transactions/{transaction_id}
>

Example request to fetch details about a Transaction:

```bash
curl 'http://127.0.0.1:3000/accounts/cfdd279d-f174-4c99-8d83-7b059e24fd25'
```

Example response:

```text
HTTP/1.1 200 OK
content-type: application/json
content-length: 291
date: Fri, 06 Jun 2025 11:47:09 GMT

{
  "created_at": "2025-06-06T11:40:16.589983Z",
  "transaction_id": "cfdd279d-f174-4c99-8d83-7b059e24fd25",
  "movement_type": "Credit",
  "lhs_account_id": "f06c7f2d-2a21-466e-a5e6-bd40b37580a4",
  "rhs_account_id": "4f543247-8160-4951-8bce-baf8e927025c",
  "description": "SEPA Transfer",
  "amount_in_cents": 10000
}
```

## Fetching journal entries

> [!INFO]
> GET /journal/transactions/{transaction_id}
>

Example request to journal entries related to a Transaction:

```bash
curl 'http://127.0.0.1:3000/journal/cfdd279d-f174-4c99-8d83-7b059e24fd25'
```

Example response:

```text
HTTP/1.1 200 OK
content-type: application/json
content-length: 504
date: Fri, 06 Jun 2025 11:49:22 GMT

[
  {
    "created_at": "2025-06-06T11:40:16.589984Z",
    "entry_id": "cbe0f005-c0c2-44d7-8483-22e426cfe05b",
    "transaction_id": "cfdd279d-f174-4c99-8d83-7b059e24fd25",
    "account_id": "f06c7f2d-2a21-466e-a5e6-bd40b37580a4",
    "movement_type": "Credit",
    "amount_in_cents": 10000
  },
  {
    "created_at": "2025-06-06T11:40:16.589984Z",
    "entry_id": "cbe0f005-c0c2-44d7-8483-22e426cfe05b",
    "transaction_id": "cfdd279d-f174-4c99-8d83-7b059e24fd25",
    "account_id": "4f543247-8160-4951-8bce-baf8e927025c",
    "movement_type": "Debit",
    "amount_in_cents": 10000
  }
]
```
