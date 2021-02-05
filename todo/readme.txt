cargo run

# コンテナからファイル取り出し
CONTAINER_ID=701a8dee43c2
FILEPATH=/todo.db
docker cp $CONTAINER_ID:$FILEPATH ./db/

docker build -t todo-app .
docker run -p 8080:8080 todo-app

