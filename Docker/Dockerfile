
FROM nginx:alpine

COPY --chown=nginx:nginx ../dist/spa /usr/share/nginx/html
RUN chmod g+rwx /var/cache/nginx /var/run /var/log/nginx

RUN sed -i.bak 's/listen\(.*\)80;/listen 9080;/' /etc/nginx/conf.d/default.conf
EXPOSE 9080

RUN sed -i.bak 's/^user/#user/' /etc/nginx/nginx.conf

RUN addgroup nginx root
USER nginx



#  test
