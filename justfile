set export := true

default: gen-env

db-pf:
  while true; do kubectl -n ipv8-dev port-forward services/psql 5432; done

db:
  USER=$(kubectl -n ipv8-dev get secret postgres.acid-game.credentials.postgresql.acid.zalan.do --template={{{{.data.username}}) && \
  PASSWORD=$(kubectl -n ipv8-dev get secret postgres.acid-game.credentials.postgresql.acid.zalan.do --template={{{{.data.password}}) && \
  USER_D=$(echo $USER | base64 -d) && \
  PASSWORD_D=$(echo $PASSWORD | base64 -d) && \
  echo postgres://$USER_D:$PASSWORD_D@localhost/postgres

psql:
  USER=$(kubectl -n ipv8-dev get secret postgres.acid-game.credentials.postgresql.acid.zalan.do --template={{{{.data.username}}) && \
  PASSWORD=$(kubectl -n ipv8-dev get secret postgres.acid-game.credentials.postgresql.acid.zalan.do --template={{{{.data.password}}) && \
  USER_D=$(echo $USER | base64 -d) && \
  PASSWORD_D=$(echo $PASSWORD | base64 -d) && \
  psql postgres://$USER_D:$PASSWORD_D@localhost/postgres

gen-env:
  USER=$(kubectl -n ipv8-dev get secret postgres.acid-game.credentials.postgresql.acid.zalan.do --template={{{{.data.username}}) && \
  PASSWORD=$(kubectl -n ipv8-dev get secret postgres.acid-game.credentials.postgresql.acid.zalan.do --template={{{{.data.password}}) && \
  USER_D=$(echo $USER | base64 -d) && \
  PASSWORD_D=$(echo $PASSWORD | base64 -d) && \
  echo "DATABASE_URI=localhost\nDATABASE_USER=$USER_D\nDATABASE_PASS=$PASSWORD_D\nDATABASE_DB=postgres" > .env

db-migrate: fix-psql-table
  USER=$(kubectl -n ipv8-dev get secret postgres.acid-game.credentials.postgresql.acid.zalan.do --template={{{{.data.username}}) && \
  PASSWORD=$(kubectl -n ipv8-dev get secret postgres.acid-game.credentials.postgresql.acid.zalan.do --template={{{{.data.password}}) && \
  USER_D=$(echo $USER | base64 -d) && \
  PASSWORD_D=$(echo $PASSWORD | base64 -d) && \
  export DATABASE_URL=postgres://$USER_D:$PASSWORD_D@localhost/postgres && \
  PGSSLMODE=disable diesel migration run  

fix-psql-table:
  USER=$(kubectl -n ipv8-dev get secret postgres.acid-game.credentials.postgresql.acid.zalan.do --template={{{{.data.username}}) && \
  PASSWORD=$(kubectl -n ipv8-dev get secret postgres.acid-game.credentials.postgresql.acid.zalan.do --template={{{{.data.password}}) && \
  USER_D=$(echo $USER | base64 -d) && \
  PASSWORD_D=$(echo $PASSWORD | base64 -d) && \
  export DATABASE_URL=postgres://$USER_D:$PASSWORD_D@localhost/postgres && \
  psql $DATABASE_URL -c "DROP TABLE IF EXISTS postgres_log CASCADE;"
