# CURL Commands for testing the Api ~Apo

BASE_URL="http://127.0.0.1:3000"
USER_ID=Set_number_here

## POST Command
curl -i -s -X POST "127.0.0.1:3000/users" \                                         
     -H "Content-Type: application/json" \
     -d '{"name":"Alice","email":"alice@example.com"}' \
     -w "\nHTTP STATUS: %{http_code}\n"

## GET Command
curl -i -s -X GET "$BASE_URL/users/2" -w "\nHTTP STATUS: %{http_code}\n"

## UPDATE Command
curl -i -s -X PUT "$BASE_URL/users/1" \
     -H "Content-Type: application/json" \
     -d '{"name":"Alice Updated","email":"alice2@example.com"}' \
     -w "\nHTTP STATUS: %{http_code}\n"

## DELETE Command
curl -i -s -X DELETE "$BASE_URL/users/$USER_ID" \
     -w "\nHTTP STATUS: %{http_code}\n"