grpcurl \
    -plaintext \
    -import-path ./proto \
    -proto slack.proto \
    -d '@' \
    [::1]:50051 slack.SlackAdaptor/PostMessage << EOF
{"channel":"foo", "message":"bar"}
EOF
