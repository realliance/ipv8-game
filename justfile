set export := true

default: gen-env

gen-env:
  USER=$(kubectl -n ipv8-dev get secret postgres.acid-game.credentials.postgresql.acid.zalan.do --template={{{{.data.username}}) && \
  PASSWORD=$(kubectl -n ipv8-dev get secret postgres.acid-game.credentials.postgresql.acid.zalan.do --template={{{{.data.password}}) && \
  USER_D=$(echo $USER | base64 -d) && \
  PASSWORD_D=$(echo $PASSWORD | base64 -d) && \
  echo "DATABASE_URI=localhost\nDATABASE_USER=$USER_D\nDATABASE_PASS=$PASSWORD_D\nDATABASE_DB=postgres" > .env