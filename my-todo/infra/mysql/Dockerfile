FROM mysql/mysql-server:8.0

ENV MYSQL_DATABASE=rust_web_app \
  MYSQL_USER=local_user \
  MYSQL_PASSWORD=secret \
  MYSQL_ROOT_PASSWORD=secret \
  TZ=Asia/Tokyo

COPY ./my.cnf /etc/my.cnf
RUN chmod 644 /etc/my.cnf
