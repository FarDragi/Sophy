FROM node:17

RUN npm install -g prisma

COPY ./prisma /prisma

CMD [ "prisma migrate deploy --skip-generate" ]
