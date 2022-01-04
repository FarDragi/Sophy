FROM node:17

RUN npm install -g prisma

COPY . .

CMD [ "prisma migrate deploy --skip-generate" ]
