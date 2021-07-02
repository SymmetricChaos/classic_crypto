// Assign a String a numerical (f64) score between 0 and 1 estimating how likely it is to be English text


// open 1grams.csv
// add up all the counts as a u64 to get a total
// close 1grams.csv
// open 1grams.csv (again)
// use the total to calculate the probability and log probability of seeing each letter
// store that information in a Vec<(String,f64,f64)>
// close 1grams.csv
// open 1gram_scores.csv
// write in the letters information from the Vec<(String,f64,f64)>
// close 1gram_scores.csv




/* with open('1gramScores.csv', 'w', newline='') as csvfile:
    writer = csv.writer(csvfile, delimiter=' ')
    
    for line in ngrams1:
        L = line.split(",")
        L[1] = int(L[1])
        logprob = log2((L[1]+1)/tot1)
        L.append(floor(logprob*100))
        L.append(logprob)
        writer.writerow(L) */