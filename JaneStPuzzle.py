### uploads the entire scrabble dictionary
with open("scrabblewords.txt", "r") as longlist:
    scrabblewords = []
    for line in longlist:
        scrabblewords.append(line.strip())


### creates a dictionary assigning letters their value
score = {}
for k in ['e', 'a', 'i', 'o', 'n', 'r', 't', 'l', 's', 'u']:
	score[k] = 1
for k in ['D', 'G']:
	score[k] = 2
for k in ['B', 'C', 'M', 'P']:
	score[k] = 3
for k in ['F', 'H', 'V', 'W', 'Y']:
	score[k] = 4
for k in ['K']:
	score[k] = 5
for k in ['J', 'X']:
	score[k] = 8
for k in ['Q', 'Z']:
	score[k] = 10


### Goes through scrabble words and puts the 
### ones with scores >= N in the good words list
goodwords = []

def get_score(word):
	return sum(map(lambda x: score[x], word))

for i in range( len(scrabblewords) ):
	highscore = True
	if get_score(scrabblewords[i]) < 36:
		highscore = False
	if highscore:
		goodwords.append( scrabblewords[i] )



for word in list(goodwords):
	for i in range(len(word)-1):
		if word[i] == word[i+1] or word[0] == 'Q':
			goodwords.remove(word)
			break
		if word[i] == 'O' and word[i+1] == 'U':
			goodwords.remove(word)
			break
		if word[i] == 'U' and word[i+1] == 'O':
			goodwords.remove(word)
			break



### Make quadruples out of good words
quadlist = []

for word1 in list(goodwords):
	for word2 in list(goodwords):
		if word1[-1] == word2[0]:
			for word3 in list(goodwords):
				if word2[-1] == word3[0] and word3 != word1:
					for word4 in list(goodwords):
						if word3[-1] == word4[0] and word2 != word4:
							templist = [word1, word2, word3, word4]
							quadlist.append(templist)


### Refine using characteristics quadruples must have
for tupl in list(quadlist):
	charset = set()
	charset2 = set()
	for ch in tupl[0]: 
		if ch not in ('A', 'E', 'I', 'O', 'U'): 
			charset.add(ch)
	for ch2 in tupl[3]:
		if ch2 not in ('A', 'E', 'I', 'O', 'U'): 
			charset2.add(ch2)
	if len(charset.intersection(charset2)) > 2:
		quadlist.remove(tupl)
	elif 'U' in (tupl[0][0], tupl[0][1], tupl[0][2], 
		tupl[0][-1], tupl[0][-2], tupl[0][-3], 
		tupl[1][0], tupl[1][1], tupl[1][2], 
		tupl[1][-1], tupl[1][-2], 
		tupl[2][0], tupl[2][1], 
		tupl[2][-1], tupl[2][-2], tupl[2][-3], 
		tupl[3][0], tupl[3][1], tupl[3][2], 
		tupl[3][-1], tupl[3][-2], tupl[3][-3]): 
		quadlist.remove(tupl)



### export the list of tuples
# with open('quadruples<32.txt', 'w') as f:
# 	for quad in quadlist:
# 		f.write("%s\n" % quad)


print(len(goodwords))

print(len(quadlist))
for q in quadlist: 
	print(q) 




