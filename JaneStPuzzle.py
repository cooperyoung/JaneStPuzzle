### uploads the entire scrabble dictionary
with open("scrabblewords.txt", "r") as longlist:
    scrabblewords = []
    for line in longlist:
        scrabblewords.append(line.strip())


### creates a dictionary assigning letters their value
score = {}
for k in ['e', 'a', 'i', 'o', 'n', 'r', 't', 'l', 's', 'u']:
	score[k] = 1
for k in ['d', 'g']:
	score[k] = 2
for k in ['b', 'c', 'm', 'p']:
	score[k] = 3
for k in ['f', 'h', 'v', 'w', 'y']:
	score[k] = 4
for k in ['k']:
	score[k] = 5
for k in ['j', 'x']:
	score[k] = 8
for k in ['q', 'z']:
	score[k] = 10


### Goes through scrabble words and puts the 
### ones with scores >= N in the good words list
goodwords = []

def get_score(word):
	return sum(map(lambda x: score[x], word))

for i in range( len(scrabblewords) ):
	highscore = True
	if get_score(scrabblewords[i]) < 32:
		highscore = False
	if highscore:
		goodwords.append( scrabblewords[i] )



for word in list(goodwords):
	for i in range(len(word)-1):
		if word[i] == word[i+1] or word[0] == 'q':
			goodwords.remove(word)
			break
		if word[i] == 'o' and word[i+1] == 'u':
			goodwords.remove(word)
			break
		if word[i] == 'u' and word[i+1] == 'o':
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
		if ch not in ('a', 'e', 'i', 'o', 'u'): 
			charset.add(ch)
	for ch2 in tupl[3]:
		if ch2 not in ('a', 'e', 'i', 'o', 'u'):
			charset2.add(ch2)
	if len(charset.intersection(charset2)) > 2:
		quadlist.remove(tupl)
	elif 'u' in (tupl[0][0], tupl[0][1], tupl[0][2], 
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
# for q in quadlist: 
# 	print(q) 




