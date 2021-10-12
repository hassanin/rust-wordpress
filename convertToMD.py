from markdownify import markdownify

# open text file in read mode
text_file = open("./report/report.html", "r")

# read whole file to a string
data = text_file.read()

md1 = markdownify(data)

print(md1)

text_file = open("./report/output.md", "wt")
n = text_file.write(md1)
text_file.close()
