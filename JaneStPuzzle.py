import string

### important variables
minvalue = 30
maxlength = 15


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



### goes through scrabble words and puts the 
### ones with scores >= N in the good words list
goodwords = []

def get_score(word):
	return sum(map(lambda x: score[x], word))

for i in range( len(scrabblewords) ):
	highscore = True
	if (get_score(scrabblewords[i]) < minvalue) or (len(scrabblewords[i]) > maxlength) :
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
		if word[i] == 'u' and word[i+1] == 'i':
			goodwords.remove(word)
			break
		if word[i] == 'i' and word[i+1] == 'u':
			goodwords.remove(word)
			break


### Put goodwords into lists according to first letter

letterlist = {'a': [], 'b': [], 'c': [], 'd': [], 'e': [], 'f': [], 'g': [], 'h': [], 'i': [], 'j': [], 
'k': [], 'l': [], 'm': [], 'n': [], 'o': [], 'p': [], 'q': [], 'r': [], 's': [], 't': [], 'u': [], 
'v': [], 'w': [], 'x': [], 'y': [], 'z': []}

for word in goodwords:
	ch = word[0]
	letterlist[ch].append(word)



### Make quadruples out of good words
quadlist = []

for word1 in list(goodwords):
	for word2 in list(letterlist[word1[-1]]):
		if word1[-1] == word2[0]:
			for word3 in list(letterlist[word2[-1]]):
				if word2[-1] == word3[0] and word3 != word1:
					for word4 in list(letterlist[word3[-1]]):
						if word3[-1] == word4[0] and word2 != word4:
							templist = [word1, word2, word3, word4]
							quadlist.append(templist)
							


### Refine list of quadruples using characteristics quadruples must have
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



######### export the list of tuples
with open('quadruples_val30len15.txt', 'w') as f:
	for quad in quadlist:
		f.write("%s\n" % quad)



############ checking to see if any last words can be improved
# for word in goodwords:
# 	goodletters = True
# 	for i in range(len(word)):
# 		if word[i] in  ['h', 'm', 'x', 'u', 'w']:
# 			goodletters = False
# 		if word[0] != 's':
# 			goodletters = False
# 	if goodletters:
# 		print(word)




print(len(quadlist))
# for q in quadlist: 
# 	print(q) 







