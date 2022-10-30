import os

import requests
from bs4 import BeautifulSoup
import re
import sqlite3

streaming_url = 'https://www.rottentomatoes.com/browse/movies_at_home/critics:certified_fresh~sort:newest?page=1'
theaters_url = 'https://www.rottentomatoes.com/browse/movies_in_theaters/critics:certified_fresh~sort:newest?page=1'
movies_folder = r'D:\Peliculas\_RottenTomatoesCertified'
browser_headers = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/63.0.3239.132 Safari/537.36 QIHU 360SE'
}
no_results_string = 'No results returned'


def db_cursor(action):
    with sqlite3.connect('movies.db') as con:
        return action(con.cursor())


def init_db():
    def create_table(cursor):
        cursor.execute(
            'CREATE TABLE IF NOT EXISTS movies (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, year TEXT, downloaded INTEGER DEFAULT 0)')

    db_cursor(create_table)


def store_movie(movie_name, movie_year):
    def insert_movie(cursor):
        if cursor.execute('SELECT * FROM movies WHERE name = ? AND year = ?', (movie_name, movie_year)).fetchone():
            return
        cursor.execute('INSERT INTO movies (name, year) VALUES (?, ?)', (movie_name, movie_year))

    db_cursor(insert_movie)


def mark_as_downloaded(movie_id):
    def update_movie(cursor):
        cursor.execute('UPDATE movies SET downloaded = 1 WHERE id = ?', (movie_id,))

    db_cursor(update_movie)


def not_downloaded_movies():
    def get_movies(cursor):
        return cursor.execute('SELECT * FROM movies WHERE downloaded = 0').fetchall()

    return db_cursor(get_movies)


def load_rotten_tomatoes_movies():
    print('Loading movies from Rotten Tomatoes...')
    streaming_page = requests.get(streaming_url)
    theaters_page = requests.get(theaters_url)

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

        print(f'Found {movie[1]} on {piratebay_url}')

        piratebay_movie_info = requests.get(piratebay_url).json()[0]
        if piratebay_movie_info['name'] != no_results_string:
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
