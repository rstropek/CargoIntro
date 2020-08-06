FROM node
COPY . /slides/
WORKDIR /slides
RUN npm install
CMD ["npm", "start", "--", "--port", "80"]
