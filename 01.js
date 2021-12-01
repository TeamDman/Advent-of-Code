temp1
// <pre style=​"word-wrap:​ break-word;​ white-space:​ pre-wrap;​">​…​</pre>​


lines=temp1.innerText.split("\n")
// (2001) ['140', '154', '165', '150', '151', '161', '172', '174', '166', '168', '176', '191', '192', '189', '190', '191', '202', '203', '206', '207', '167', '179', '204', '206', '208', '209', '216', '198', '200', '207', '217', '218', '223', '227', '228', '243', '241', '246', '278', '255', '256', '273', '295', '291', '288', '290', '303', '325', '301', '292', '291', '309', '313', '315', '319', '339', '340', '327', '336', '335', '340', '331', '334', '316', '317', '323', '324', '323', '312', '298', '291', '285', '288', '266', '267', '266', '258', '259', '257', '259', '250', '265', '270', '267', '271', '272', '277', '281', '277', '284', '291', '283', '284', '291', '292', '278', '285', '296', '294', '297', …]

lines1 = lines.map(x => parseInt(x))
// (2001) [140, 154, 165, 150, 151, 161, 172, 174, 166, 168, 176, 191, 192, 189, 190, 191, 202, 203, 206, 207, 167, 179, 204, 206, 208, 209, 216, 198, 200, 207, 217, 218, 223, 227, 228, 243, 241, 246, 278, 255, 256, 273, 295, 291, 288, 290, 303, 325, 301, 292, 291, 309, 313, 315, 319, 339, 340, 327, 336, 335, 340, 331, 334, 316, 317, 323, 324, 323, 312, 298, 291, 285, 288, 266, 267, 266, 258, 259, 257, 259, 250, 265, 270, 267, 271, 272, 277, 281, 277, 284, 291, 283, 284, 291, 292, 278, 285, 296, 294, 297, …]

lines2 = lines1.map((v,i) => v > lines1?.[i-1]??v)
// (2001) [false, true, true, false, true, true, true, true, false, true, true, true, true, false, true, true, true, true, true, true, false, true, true, true, true, true, true, false, true, true, true, true, true, true, true, true, false, true, true, false, true, true, true, false, false, true, true, true, false, false, false, true, true, true, true, true, true, false, true, false, true, false, true, false, true, true, true, false, false, false, false, false, true, false, true, false, false, true, false, true, false, true, true, false, true, true, true, true, false, true, true, false, true, true, true, false, true, true, false, true, …]

lines3 = lines2.filter(x => x)
// (1298) [true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, …]

result = lines3.length