package main

//Assumption: all numbers after sum are within int range, so ignoring overflow cases due to sum
type Movie struct {
	movieId  int
	playTime int //In minutes
}

func sumOfMovieTime(movies []Movie) int {
	total := 0
	for _, movie := range movies {
		total += movie.playTime
	}

	return total
}

func findMovieCombinationHelper(movies []Movie, target int, resultMovieCombination []Movie,
	matchedMovies *[][]Movie) {
	totalPlayTime := sumOfMovieTime(resultMovieCombination)
	if totalPlayTime == target {
		*matchedMovies = append(*matchedMovies, resultMovieCombination)
	}
	if totalPlayTime >= target {
		return
	}

	for i, movie := range movies {
		remainingMovies := movies[i+1:]
		findMovieCombinationHelper(remainingMovies, target, append(resultMovieCombination, movie), matchedMovies)
	}
}

func findMovieCombination(movies []Movie, target int) [][]Movie {
	var matchedMovies [][]Movie
	findMovieCombinationHelper(movies, target, []Movie{}, &matchedMovies)
	return matchedMovies
}
