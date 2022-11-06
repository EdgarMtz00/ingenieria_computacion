import os

import requests
from bs4 import BeautifulSoup
import re
import sqlite3

STREAMING_URL = 'https://www.rottentomatoes.com/browse/movies_at_home/critics:certified_fresh~sort:newest?page=5'
THEATERS_URL = 'https://www.rottentomatoes.com/browse/movies_in_theaters/critics:certified_fresh~sort:newest?page=5'
MOVIES_FOLDER = r'D:\Peliculas\_RottenTomatoesCertified'
HEADERS = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/63.0.3239.132 Safari/537.36 QIHU 360SE'
}
NO_RESULTS = 'No results returned'

CREATE_TABLE = 'CREATE TABLE IF NOT EXISTS movies (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, year TEXT, downloaded INTEGER DEFAULT 0)'
FIND_QUERY = 'SELECT id FROM movies WHERE name = ? AND year = ?'
INSERT_QUERY = 'INSERT INTO movies (name, year) VALUES (?, ?)'
MARK_AS_DOWNLOADED = 'UPDATE movies SET downloaded = 1 WHERE id = ?'
FIND_NOT_DOWNLOADED = 'SELECT * FROM movies WHERE downloaded = 0'


def db_cursor(action):
    with sqlite3.connect('movies.db') as con:
        return action(con.cursor())


def init_db():
    db_cursor(lambda cursor: cursor.execute(CREATE_TABLE))


def store_movie(movie_name, movie_year):
    def insert_movie(cursor):
        if cursor.execute(FIND_QUERY, (movie_name, movie_year)).fetchone():
            return
        cursor.execute(INSERT_QUERY, (movie_name, movie_year))

    db_cursor(insert_movie)


def mark_as_downloaded(movie_id):
    db_cursor(lambda cursor: cursor.execute(MARK_AS_DOWNLOADED, (movie_id,)))


def not_downloaded_movies():
    return db_cursor(lambda cursor: cursor.execute(FIND_NOT_DOWNLOADED).fetchall())


def load_rotten_tomatoes_movies():
    print('Loading movies from Rotten Tomatoes...')
    streaming_page = requests.get(STREAMING_URL)
    theaters_page = requests.get(THEATERS_URL)

    streaming_movies_info = BeautifulSoup(streaming_page.content, 'lxml') \
        .find_all('div', {'data-qa': 'discovery-media-list-item-caption'})
    theaters_movies_info = BeautifulSoup(theaters_page.content, 'lxml') \
        .find_all('div', {'data-qa': 'discovery-media-list-item-caption'})

    for movie in streaming_movies_info + theaters_movies_info:
        movie_name = movie.find('span', {'data-qa': 'discovery-media-list-item-title'}).text.strip()
        if movie_name:
            movie_date = movie.find('span', {'data-qa': 'discovery-media-list-item-start-date'}).text.strip()
            movie_year = re.findall(r'[0-9]{4}', movie_date)[0]
            store_movie(movie_name, movie_year)


def download_piratebay_movies(movies):
    print('Downloading movies from PirateBay...')
    for movie in movies:
        movie_name = movie[1].replace(' ', '+')
        movie_year = movie[2]

        piratebay_url = f'https://apibay.org/q.php?q={movie_name}+{movie_year}'

        piratebay_movie_info = requests.get(piratebay_url).json()[0]
        if piratebay_movie_info['name'] != NO_RESULTS:
            magnet_link = f'magnet:?xt=urn:btih:{piratebay_movie_info["info_hash"]}&dn={piratebay_movie_info["name"]}'
            print(f'Downloading {movie[1]} from {magnet_link}')
            os.startfile(magnet_link)
            mark_as_downloaded(movie[0])
        else:
            print(f'Could not find {movie[1]} on PirateBay')
        print('\n')


def main():
    init_db()
    load_rotten_tomatoes_movies()
    movies = not_downloaded_movies()
    download_piratebay_movies(movies)


if __name__ == '__main__':
    main()
