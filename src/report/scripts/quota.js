
$(function(){


  init();

})
function init(){

// ['厦门第一医院','厦门中山医院','厦门中医院','厦门第五医院',];

  var myColor = ['#1089E7', '#F57474', '#56D0E3', '#F8B448', '#8B78F6'];
  var name = 'X-lab2017/open-digger';
  
  var data1 = ["2020-08", "2020-09", "2020-10", "2020-11", "2020-12", "2021-01", "2021-02", "2021-03", "2021-04", "2021-05", "2021-06", "2021-07", "2021-08", "2021-09", "2021-10", "2021-10-raw", "2021-11", "2021-12", "2022-01", "2022-02", "2022-03", "2022-04", "2022-05", "2022-06", "2022-07", "2022-08", "2022-09", "2022-10", "2022-11", "2022-12", "2023-01", "2023-02", "2023-03", "2023-04", "2023-05", "2023-06"];
  var data2 = [10, 23, 33, 37, 41, 42, 49, 52, 58, 62, 67, 71, 72, 74, 78, 79, 86, 89, 94, 99, 201, 204, 208, 209, 211, 218, 228, 237, 246, 249, 250, 258, 270, 276, 278, 281];
  var data3 = ["2020-08", "2020-09", "2020-10", "2020-11", "2020-12", "2021-01", "2021-02", "2021-03", "2021-04", "2021-06", "2021-07", "2021-08", "2021-09", "2021-11", "2022-03", "2022-04", "2022-05", "2022-06", "2022-07", "2022-08", "2022-09", "2022-10", "2022-11", "2022-12", "2023-01", "2023-02", "2023-03", "2023-04", "2023-05", "2023-06"];
  var data4 = [4, 8, 9, 11, 21, 25, 28, 30, 32, 35, 36, 37, 38, 42, 43, 44, 46, 49, 52, 54, 57, 66, 69, 71, 73, 77, 81, 86, 89, 91];
  var data5 = ["2020-08", "2020-09", "2020-10", "2020-11", "2020-12", "2021-01", "2021-02", "2021-03", "2021-04", "2021-05", "2021-06", "2021-07", "2021-08", "2021-09", "2021-10", "2021-10-raw", "2021-11", "2021-12", "2022-01", "2022-02", "2022-03", "2022-04", "2022-05", "2022-06", "2022-07", "2022-08", "2022-09", "2022-10", "2022-11", "2022-12", "2023-01", "2023-02", "2023-03", "2023-04", "2023-05", "2023-06"];
  var data6 = [4.5, 9.41, 15, 21.31, 31.27, 41.879999999999995, 48.16, 52.3, 56.739999999999995, 60.99999999999999, 67.46, 72.3, 76.23, 79.57000000000001, 82.57000000000001, 85.41000000000001, 88.30000000000001, 91.63000000000001, 96.34, 101.21000000000001, 107.27000000000001, 111.03000000000002, 115.17000000000002, 122.84000000000002, 132.01000000000002, 140.54000000000002, 150.50000000000003, 162.34000000000003, 176.99000000000004, 196.35000000000002, 216.25000000000003, 256.73, 278.78000000000003, 297.57000000000005, 315.99000000000007, 330.2900000000001];
  var data7 = ["2020-08", "2020-09", "2020-10", "2020-11", "2020-12", "2021-01", "2021-02", "2021-03", "2021-04", "2021-05", "2021-06", "2021-07", "2021-08", "2021-09", "2021-10", "2021-10-raw", "2021-11", "2021-12", "2022-01", "2022-02", "2022-03", "2022-04", "2022-05", "2022-06", "2022-07", "2022-08", "2022-09", "2022-10", "2022-11", "2022-12", "2023-01", "2023-02", "2023-03", "2023-04", "2023-05", "2023-06"];
  var data8 = [35.29, 78.31, 106.84, 137.23000000000002, 214.44, 269.24, 291.67, 305.91, 336.33000000000004, 356.26000000000005, 396.80000000000007, 413.51000000000005, 425.87000000000006, 429.87000000000006, 443.87000000000006, 445.2800000000001, 464.2200000000001, 490.5300000000001, 516.6100000000001, 533.6500000000001, 578.8700000000001, 601.6800000000001, 626.5400000000001, 676.2800000000001, 716.58, 752.63, 798.36, 855.4200000000001, 912.7500000000001, 971.2100000000002, 1027.41, 1150.72, 1189.78, 1242.21, 1291.13, 1321.5600000000002];
  var data9 = [["snyk-matt/goof-platform", 138.3], ["sanjeevi-mariappan/tutorials", 67.32], ["sathishcyberintelsys/skf-labsss", 59.34], ["Centaurioun/raycast-extensions", 58.61], ["NOUIY/aws-sdk-java", 55.56], ["X-lab2017/oss101", 54.33], ["giffgaff/tutorials", 43.09], ["X-lab2017/open-digger", 40.84], ["harrisonho99/react-native-windows-samples", 39.95], ["bogarin/tutorials", 36.43], ["gruffwizard/tutorials", 36.01], ["NOUIY/docusaurus", 35.62], ["radhakrishna4687/tutorials", 33.57], ["xcirel/lambda-refarch-webapp", 32.89], ["bluecrystalsign/tutorials", 31.88], ["seshgirik/tutorials", 31.52], ["pawel383/tutorials", 30.73], ["xmk-dev/issue-tracker-api", 30.2], ["Mhmonicox/docs-1", 27.4], ["Balantion2020/Balantion", 27.37], ["hypertrons/hypertrons-crx", 26.21], ["s-w-high/my-react-js-tutorials", 25.49], ["NOUIY/aws-sdk-js", 25.33], ["arniebilloo/vulhub", 25.21], ["nitaandreea/tutorials", 24.25], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2", 23.26], ["TuGraph-family/tugraph-db", 20.12], ["X-lab2017/open-wonderland", 13.11], ["X-lab2017/open-leaderboard", 5.08], ["X-lab2017/od-api", 2.99], ["X-lab2017/open-research", 1.48]];
  var data10 = [["NOUIY/aws-sdk-java","NOUIY/docusaurus", 41.59], ["NOUIY/aws-sdk-java","NOUIY/aws-sdk-js", 37.06], ["NOUIY/aws-sdk-js","NOUIY/docusaurus", 34.32], ["snyk-matt/goof-platform","sanjeevi-mariappan/tutorials", 32.5], ["sathishcyberintelsys/skf-labsss","snyk-matt/goof-platform", 30.43], ["NOUIY/aws-sdk-java","snyk-matt/goof-platform", 30.02], ["Centaurioun/raycast-extensions","snyk-matt/goof-platform", 29.54], ["xcirel/lambda-refarch-webapp","snyk-matt/goof-platform", 28.43], ["snyk-matt/goof-platform","harrisonho99/react-native-windows-samples", 28.31], ["sathishcyberintelsys/skf-labsss","sanjeevi-mariappan/tutorials", 27.89], ["NOUIY/aws-sdk-java","sanjeevi-mariappan/tutorials", 27.54], ["Centaurioun/raycast-extensions","sanjeevi-mariappan/tutorials", 27.14], ["seshgirik/tutorials","snyk-matt/goof-platform", 26.71], ["NOUIY/docusaurus","snyk-matt/goof-platform", 26.67], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","snyk-matt/goof-platform", 26.5], ["arniebilloo/vulhub","snyk-matt/goof-platform", 26.37], ["xcirel/lambda-refarch-webapp","sanjeevi-mariappan/tutorials", 26.2], ["harrisonho99/react-native-windows-samples","sanjeevi-mariappan/tutorials", 26.1], ["sathishcyberintelsys/skf-labsss","NOUIY/aws-sdk-java", 26.04], ["X-lab2017/open-wonderland","X-lab2017/open-digger", 26.01], ["snyk-matt/goof-platform","xmk-dev/issue-tracker-api", 25.88], ["sathishcyberintelsys/skf-labsss","Centaurioun/raycast-extensions", 25.68], ["Mhmonicox/docs-1","snyk-matt/goof-platform", 25.57], ["Centaurioun/raycast-extensions","NOUIY/aws-sdk-java", 25.39], ["snyk-matt/goof-platform","s-w-high/my-react-js-tutorials", 25.39], ["sathishcyberintelsys/skf-labsss","xcirel/lambda-refarch-webapp", 24.84], ["bogarin/tutorials","snyk-matt/goof-platform", 24.74], ["sathishcyberintelsys/skf-labsss","harrisonho99/react-native-windows-samples", 24.74], ["seshgirik/tutorials","sanjeevi-mariappan/tutorials", 24.73], ["giffgaff/tutorials","snyk-matt/goof-platform", 24.72], ["NOUIY/docusaurus","sanjeevi-mariappan/tutorials", 24.7], ["xcirel/lambda-refarch-webapp","NOUIY/aws-sdk-java", 24.56], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","sanjeevi-mariappan/tutorials", 24.56], ["snyk-matt/goof-platform","nitaandreea/tutorials", 24.53], ["Balantion2020/Balantion","snyk-matt/goof-platform", 24.53], ["NOUIY/aws-sdk-java","harrisonho99/react-native-windows-samples", 24.47], ["arniebilloo/vulhub","sanjeevi-mariappan/tutorials", 24.44], ["Centaurioun/raycast-extensions","xcirel/lambda-refarch-webapp", 24.25], ["Centaurioun/raycast-extensions","harrisonho99/react-native-windows-samples", 24.15], ["gruffwizard/tutorials","snyk-matt/goof-platform", 24.12], ["sanjeevi-mariappan/tutorials","xmk-dev/issue-tracker-api", 24.02], ["snyk-matt/goof-platform","bluecrystalsign/tutorials", 23.98], ["snyk-matt/goof-platform","pawel383/tutorials", 23.79], ["Mhmonicox/docs-1","sanjeevi-mariappan/tutorials", 23.75], ["radhakrishna4687/tutorials","snyk-matt/goof-platform", 23.69], ["s-w-high/my-react-js-tutorials","sanjeevi-mariappan/tutorials", 23.6], ["NOUIY/aws-sdk-js","snyk-matt/goof-platform", 23.57], ["sathishcyberintelsys/skf-labsss","seshgirik/tutorials", 23.52], ["sathishcyberintelsys/skf-labsss","NOUIY/docusaurus", 23.49], ["xcirel/lambda-refarch-webapp","harrisonho99/react-native-windows-samples", 23.41], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","sathishcyberintelsys/skf-labsss", 23.36], ["seshgirik/tutorials","NOUIY/aws-sdk-java", 23.27], ["sathishcyberintelsys/skf-labsss","arniebilloo/vulhub", 23.25], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","NOUIY/aws-sdk-java", 23.11], ["bogarin/tutorials","sanjeevi-mariappan/tutorials", 23.04], ["giffgaff/tutorials","sanjeevi-mariappan/tutorials", 23.02], ["NOUIY/aws-sdk-java","arniebilloo/vulhub", 23.01], ["Centaurioun/raycast-extensions","seshgirik/tutorials", 22.98], ["Centaurioun/raycast-extensions","NOUIY/docusaurus", 22.95], ["sathishcyberintelsys/skf-labsss","xmk-dev/issue-tracker-api", 22.87], ["X-lab2017/oss101","hypertrons/hypertrons-crx", 22.86], ["sanjeevi-mariappan/tutorials","nitaandreea/tutorials", 22.86], ["Balantion2020/Balantion","sanjeevi-mariappan/tutorials", 22.85], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","Centaurioun/raycast-extensions", 22.83], ["Centaurioun/raycast-extensions","arniebilloo/vulhub", 22.73], ["sathishcyberintelsys/skf-labsss","Mhmonicox/docs-1", 22.63], ["NOUIY/aws-sdk-java","xmk-dev/issue-tracker-api", 22.63], ["gruffwizard/tutorials","sanjeevi-mariappan/tutorials", 22.5], ["sathishcyberintelsys/skf-labsss","s-w-high/my-react-js-tutorials", 22.49], ["Mhmonicox/docs-1","NOUIY/aws-sdk-java", 22.4], ["sanjeevi-mariappan/tutorials","bluecrystalsign/tutorials", 22.37], ["Centaurioun/raycast-extensions","xmk-dev/issue-tracker-api", 22.36], ["xcirel/lambda-refarch-webapp","seshgirik/tutorials", 22.3], ["xcirel/lambda-refarch-webapp","NOUIY/docusaurus", 22.28], ["NOUIY/aws-sdk-java","s-w-high/my-react-js-tutorials", 22.26], ["seshgirik/tutorials","harrisonho99/react-native-windows-samples", 22.23], ["sanjeevi-mariappan/tutorials","pawel383/tutorials", 22.21], ["NOUIY/docusaurus","harrisonho99/react-native-windows-samples", 22.2], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","xcirel/lambda-refarch-webapp", 22.16], ["Centaurioun/raycast-extensions","Mhmonicox/docs-1", 22.13], ["radhakrishna4687/tutorials","sanjeevi-mariappan/tutorials", 22.12], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","harrisonho99/react-native-windows-samples", 22.08], ["xcirel/lambda-refarch-webapp","arniebilloo/vulhub", 22.06], ["NOUIY/aws-sdk-js","sanjeevi-mariappan/tutorials", 22.01], ["Centaurioun/raycast-extensions","s-w-high/my-react-js-tutorials", 22], ["arniebilloo/vulhub","harrisonho99/react-native-windows-samples", 21.99], ["sathishcyberintelsys/skf-labsss","bogarin/tutorials", 21.98], ["sathishcyberintelsys/skf-labsss","giffgaff/tutorials", 21.96], ["sathishcyberintelsys/skf-labsss","nitaandreea/tutorials", 21.81], ["sathishcyberintelsys/skf-labsss","Balantion2020/Balantion", 21.81], ["bogarin/tutorials","NOUIY/aws-sdk-java", 21.76], ["giffgaff/tutorials","NOUIY/aws-sdk-java", 21.74], ["xcirel/lambda-refarch-webapp","xmk-dev/issue-tracker-api", 21.72], ["harrisonho99/react-native-windows-samples","xmk-dev/issue-tracker-api", 21.65], ["NOUIY/aws-sdk-java","nitaandreea/tutorials", 21.6], ["NOUIY/aws-sdk-java","Balantion2020/Balantion", 21.59], ["Centaurioun/raycast-extensions","bogarin/tutorials", 21.51], ["xcirel/lambda-refarch-webapp","Mhmonicox/docs-1", 21.5], ["Centaurioun/raycast-extensions","giffgaff/tutorials", 21.49], ["sathishcyberintelsys/skf-labsss","gruffwizard/tutorials", 21.49], ["Mhmonicox/docs-1","harrisonho99/react-native-windows-samples", 21.43], ["xcirel/lambda-refarch-webapp","s-w-high/my-react-js-tutorials", 21.38], ["sathishcyberintelsys/skf-labsss","bluecrystalsign/tutorials", 21.37], ["Centaurioun/raycast-extensions","nitaandreea/tutorials", 21.35], ["Centaurioun/raycast-extensions","Balantion2020/Balantion", 21.35], ["harrisonho99/react-native-windows-samples","s-w-high/my-react-js-tutorials", 21.31], ["NOUIY/aws-sdk-java","gruffwizard/tutorials", 21.28], ["sathishcyberintelsys/skf-labsss","pawel383/tutorials", 21.22], ["seshgirik/tutorials","NOUIY/docusaurus", 21.21], ["NOUIY/aws-sdk-java","bluecrystalsign/tutorials", 21.17], ["sathishcyberintelsys/skf-labsss","radhakrishna4687/tutorials", 21.14], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","seshgirik/tutorials", 21.1], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","NOUIY/docusaurus", 21.08], ["Centaurioun/raycast-extensions","gruffwizard/tutorials", 21.04], ["sathishcyberintelsys/skf-labsss","NOUIY/aws-sdk-js", 21.04], ["NOUIY/aws-sdk-java","pawel383/tutorials", 21.02], ["seshgirik/tutorials","arniebilloo/vulhub", 21.01], ["NOUIY/docusaurus","arniebilloo/vulhub", 20.99], ["NOUIY/aws-sdk-java","radhakrishna4687/tutorials", 20.94], ["Centaurioun/raycast-extensions","bluecrystalsign/tutorials", 20.93], ["xcirel/lambda-refarch-webapp","bogarin/tutorials", 20.91], ["xcirel/lambda-refarch-webapp","giffgaff/tutorials", 20.9], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","arniebilloo/vulhub", 20.88], ["bogarin/tutorials","harrisonho99/react-native-windows-samples", 20.85], ["giffgaff/tutorials","harrisonho99/react-native-windows-samples", 20.83], ["Centaurioun/raycast-extensions","pawel383/tutorials", 20.79], ["xcirel/lambda-refarch-webapp","nitaandreea/tutorials", 20.77], ["xcirel/lambda-refarch-webapp","Balantion2020/Balantion", 20.76], ["Centaurioun/raycast-extensions","radhakrishna4687/tutorials", 20.71], ["seshgirik/tutorials","xmk-dev/issue-tracker-api", 20.7], ["harrisonho99/react-native-windows-samples","nitaandreea/tutorials", 20.7], ["Balantion2020/Balantion","harrisonho99/react-native-windows-samples", 20.69], ["NOUIY/docusaurus","xmk-dev/issue-tracker-api", 20.68], ["Centaurioun/raycast-extensions","NOUIY/aws-sdk-js", 20.62], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","xmk-dev/issue-tracker-api", 20.58], ["seshgirik/tutorials","Mhmonicox/docs-1", 20.51], ["arniebilloo/vulhub","xmk-dev/issue-tracker-api", 20.49], ["Mhmonicox/docs-1","NOUIY/docusaurus", 20.48], ["xcirel/lambda-refarch-webapp","gruffwizard/tutorials", 20.47], ["gruffwizard/tutorials","harrisonho99/react-native-windows-samples", 20.4], ["seshgirik/tutorials","s-w-high/my-react-js-tutorials", 20.39], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","Mhmonicox/docs-1", 20.38], ["xcirel/lambda-refarch-webapp","bluecrystalsign/tutorials", 20.37], ["NOUIY/docusaurus","s-w-high/my-react-js-tutorials", 20.37], ["X-lab2017/oss101","X-lab2017/open-wonderland", 20.3], ["Mhmonicox/docs-1","arniebilloo/vulhub", 20.3], ["harrisonho99/react-native-windows-samples","bluecrystalsign/tutorials", 20.3], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","s-w-high/my-react-js-tutorials", 20.27], ["xcirel/lambda-refarch-webapp","pawel383/tutorials", 20.23], ["arniebilloo/vulhub","s-w-high/my-react-js-tutorials", 20.19], ["X-lab2017/open-digger","hypertrons/hypertrons-crx", 20.17], ["harrisonho99/react-native-windows-samples","pawel383/tutorials", 20.17], ["xcirel/lambda-refarch-webapp","radhakrishna4687/tutorials", 20.16], ["radhakrishna4687/tutorials","harrisonho99/react-native-windows-samples", 20.09], ["xcirel/lambda-refarch-webapp","NOUIY/aws-sdk-js", 20.07], ["Mhmonicox/docs-1","xmk-dev/issue-tracker-api", 20.01], ["NOUIY/aws-sdk-js","harrisonho99/react-native-windows-samples", 20.01], ["seshgirik/tutorials","bogarin/tutorials", 19.97], ["bogarin/tutorials","NOUIY/docusaurus", 19.95], ["seshgirik/tutorials","giffgaff/tutorials", 19.95], ["giffgaff/tutorials","NOUIY/docusaurus", 19.93], ["s-w-high/my-react-js-tutorials","xmk-dev/issue-tracker-api", 19.9], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","bogarin/tutorials", 19.85], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","giffgaff/tutorials", 19.84], ["seshgirik/tutorials","nitaandreea/tutorials", 19.83], ["seshgirik/tutorials","Balantion2020/Balantion", 19.83], ["NOUIY/docusaurus","nitaandreea/tutorials", 19.81], ["Balantion2020/Balantion","NOUIY/docusaurus", 19.81], ["bogarin/tutorials","arniebilloo/vulhub", 19.77], ["giffgaff/tutorials","arniebilloo/vulhub", 19.76], ["Mhmonicox/docs-1","s-w-high/my-react-js-tutorials", 19.72], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","nitaandreea/tutorials", 19.72], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","Balantion2020/Balantion", 19.71], ["Balantion2020/Balantion","arniebilloo/vulhub", 19.64], ["arniebilloo/vulhub","nitaandreea/tutorials", 19.64], ["seshgirik/tutorials","gruffwizard/tutorials", 19.56], ["NOUIY/docusaurus","gruffwizard/tutorials", 19.54], ["bogarin/tutorials","xmk-dev/issue-tracker-api", 19.5], ["giffgaff/tutorials","xmk-dev/issue-tracker-api", 19.49], ["seshgirik/tutorials","bluecrystalsign/tutorials", 19.47], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","gruffwizard/tutorials", 19.45], ["NOUIY/docusaurus","bluecrystalsign/tutorials", 19.45], ["arniebilloo/vulhub","gruffwizard/tutorials", 19.38], ["xmk-dev/issue-tracker-api","nitaandreea/tutorials", 19.37], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","bluecrystalsign/tutorials", 19.36], ["Balantion2020/Balantion","xmk-dev/issue-tracker-api", 19.36], ["seshgirik/tutorials","pawel383/tutorials", 19.35], ["NOUIY/docusaurus","pawel383/tutorials", 19.33], ["Mhmonicox/docs-1","bogarin/tutorials", 19.32], ["Mhmonicox/docs-1","giffgaff/tutorials", 19.31], ["seshgirik/tutorials","radhakrishna4687/tutorials", 19.28], ["arniebilloo/vulhub","bluecrystalsign/tutorials", 19.28], ["NOUIY/docusaurus","radhakrishna4687/tutorials", 19.25], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","pawel383/tutorials", 19.24], ["bogarin/tutorials","s-w-high/my-react-js-tutorials", 19.22], ["giffgaff/tutorials","s-w-high/my-react-js-tutorials", 19.21], ["Mhmonicox/docs-1","nitaandreea/tutorials", 19.2], ["seshgirik/tutorials","NOUIY/aws-sdk-js", 19.2], ["Mhmonicox/docs-1","Balantion2020/Balantion", 19.19], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","radhakrishna4687/tutorials", 19.17], ["arniebilloo/vulhub","pawel383/tutorials", 19.16], ["gruffwizard/tutorials","xmk-dev/issue-tracker-api", 19.11], ["s-w-high/my-react-js-tutorials","nitaandreea/tutorials", 19.1], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","NOUIY/aws-sdk-js", 19.09], ["Balantion2020/Balantion","s-w-high/my-react-js-tutorials", 19.09], ["arniebilloo/vulhub","radhakrishna4687/tutorials", 19.09], ["xmk-dev/issue-tracker-api","bluecrystalsign/tutorials", 19.02], ["NOUIY/aws-sdk-js","arniebilloo/vulhub", 19.02], ["X-lab2017/open-wonderland","hypertrons/hypertrons-crx", 18.99], ["Mhmonicox/docs-1","gruffwizard/tutorials", 18.94], ["xmk-dev/issue-tracker-api","pawel383/tutorials", 18.91], ["Mhmonicox/docs-1","bluecrystalsign/tutorials", 18.86], ["gruffwizard/tutorials","s-w-high/my-react-js-tutorials", 18.85], ["radhakrishna4687/tutorials","xmk-dev/issue-tracker-api", 18.84], ["bogarin/tutorials","giffgaff/tutorials", 18.83], ["X-lab2017/oss101","X-lab2017/open-digger", 18.76], ["s-w-high/my-react-js-tutorials","bluecrystalsign/tutorials", 18.76], ["NOUIY/aws-sdk-js","xmk-dev/issue-tracker-api", 18.76], ["Mhmonicox/docs-1","pawel383/tutorials", 18.74], ["bogarin/tutorials","nitaandreea/tutorials", 18.73], ["bogarin/tutorials","Balantion2020/Balantion", 18.72], ["giffgaff/tutorials","nitaandreea/tutorials", 18.71], ["giffgaff/tutorials","Balantion2020/Balantion", 18.71], ["Mhmonicox/docs-1","radhakrishna4687/tutorials", 18.67], ["s-w-high/my-react-js-tutorials","pawel383/tutorials", 18.65], ["Mhmonicox/docs-1","NOUIY/aws-sdk-js", 18.6], ["Balantion2020/Balantion","nitaandreea/tutorials", 18.6], ["radhakrishna4687/tutorials","s-w-high/my-react-js-tutorials", 18.58], ["NOUIY/aws-sdk-js","s-w-high/my-react-js-tutorials", 18.51], ["bogarin/tutorials","gruffwizard/tutorials", 18.48], ["giffgaff/tutorials","gruffwizard/tutorials", 18.47], ["bogarin/tutorials","bluecrystalsign/tutorials", 18.4], ["giffgaff/tutorials","bluecrystalsign/tutorials", 18.39], ["gruffwizard/tutorials","nitaandreea/tutorials", 18.37], ["Balantion2020/Balantion","gruffwizard/tutorials", 18.36], ["bogarin/tutorials","pawel383/tutorials", 18.29], ["bluecrystalsign/tutorials","nitaandreea/tutorials", 18.29], ["giffgaff/tutorials","pawel383/tutorials", 18.28], ["Balantion2020/Balantion","bluecrystalsign/tutorials", 18.28], ["bogarin/tutorials","radhakrishna4687/tutorials", 18.23], ["giffgaff/tutorials","radhakrishna4687/tutorials", 18.22], ["pawel383/tutorials","nitaandreea/tutorials", 18.18], ["Balantion2020/Balantion","pawel383/tutorials", 18.17], ["bogarin/tutorials","NOUIY/aws-sdk-js", 18.16], ["giffgaff/tutorials","NOUIY/aws-sdk-js", 18.15], ["radhakrishna4687/tutorials","nitaandreea/tutorials", 18.12], ["Balantion2020/Balantion","radhakrishna4687/tutorials", 18.11], ["gruffwizard/tutorials","bluecrystalsign/tutorials", 18.06], ["NOUIY/aws-sdk-js","nitaandreea/tutorials", 18.05], ["NOUIY/aws-sdk-js","Balantion2020/Balantion", 18.04], ["gruffwizard/tutorials","pawel383/tutorials", 17.95], ["gruffwizard/tutorials","radhakrishna4687/tutorials", 17.89], ["bluecrystalsign/tutorials","pawel383/tutorials", 17.87], ["NOUIY/aws-sdk-js","gruffwizard/tutorials", 17.82], ["radhakrishna4687/tutorials","bluecrystalsign/tutorials", 17.81], ["NOUIY/aws-sdk-js","bluecrystalsign/tutorials", 17.74], ["radhakrishna4687/tutorials","pawel383/tutorials", 17.71], ["NOUIY/aws-sdk-js","pawel383/tutorials", 17.64], ["NOUIY/aws-sdk-js","radhakrishna4687/tutorials", 17.58], ["X-lab2017/open-wonderland","X-lab2017/open-research", 17.53], ["X-lab2017/oss101","X-lab2017/open-leaderboard", 15.76], ["X-lab2017/open-leaderboard","hypertrons/hypertrons-crx", 14.05], ["X-lab2017/open-research","X-lab2017/open-digger", 12.39], ["X-lab2017/oss101","X-lab2017/open-research", 10.26], ["X-lab2017/oss101","X-lab2017/od-api", 9.34], ["X-lab2017/open-leaderboard","X-lab2017/open-wonderland", 9.22], ["X-lab2017/od-api","X-lab2017/open-leaderboard", 8.08], ["X-lab2017/od-api","X-lab2017/open-wonderland", 8.07], ["X-lab2017/open-research","hypertrons/hypertrons-crx", 6.52], ["X-lab2017/open-leaderboard","X-lab2017/open-digger", 6.3], ["TuGraph-family/tugraph-db","X-lab2017/open-digger", 6.19], ["TuGraph-family/tugraph-db","hypertrons/hypertrons-crx", 6.15], ["X-lab2017/od-api","hypertrons/hypertrons-crx", 5.26], ["snyk-matt/goof-platform","X-lab2017/open-digger", 5.2], ["sanjeevi-mariappan/tutorials","X-lab2017/open-digger", 5.12], ["X-lab2017/od-api","X-lab2017/open-digger", 5.07], ["sathishcyberintelsys/skf-labsss","X-lab2017/open-digger", 5.07], ["NOUIY/aws-sdk-java","X-lab2017/open-digger", 5.05], ["Centaurioun/raycast-extensions","X-lab2017/open-digger", 5.04], ["xcirel/lambda-refarch-webapp","X-lab2017/open-digger", 5.01], ["harrisonho99/react-native-windows-samples","X-lab2017/open-digger", 5], ["seshgirik/tutorials","X-lab2017/open-digger", 4.95], ["NOUIY/docusaurus","X-lab2017/open-digger", 4.95], ["info-zezotechnology-com/cli-snyk-zezo-f3c86dc2","X-lab2017/open-digger", 4.94], ["arniebilloo/vulhub","X-lab2017/open-digger", 4.94], ["X-lab2017/open-digger","xmk-dev/issue-tracker-api", 4.92], ["Mhmonicox/docs-1","X-lab2017/open-digger", 4.91], ["s-w-high/my-react-js-tutorials","X-lab2017/open-digger", 4.9], ["bogarin/tutorials","X-lab2017/open-digger", 4.88], ["giffgaff/tutorials","X-lab2017/open-digger", 4.88], ["X-lab2017/open-digger","nitaandreea/tutorials", 4.87], ["Balantion2020/Balantion","X-lab2017/open-digger", 4.87], ["gruffwizard/tutorials","X-lab2017/open-digger", 4.85], ["X-lab2017/open-digger","bluecrystalsign/tutorials", 4.85], ["radhakrishna4687/tutorials","X-lab2017/open-digger", 4.84], ["X-lab2017/open-digger","pawel383/tutorials", 4.84], ["NOUIY/aws-sdk-js","X-lab2017/open-digger", 4.83], ["X-lab2017/open-leaderboard","X-lab2017/open-research", 4.51], ["X-lab2017/od-api","X-lab2017/open-research", 3.44], ["TuGraph-family/tugraph-db","X-lab2017/open-wonderland", 3.24], ["X-lab2017/oss101","TuGraph-family/tugraph-db", 0.88]];

  var graphData = {
    "nodes": data9,
    "edges": data10
};

// 转换为 ECharts 需要的节点和边格式
var nodes = graphData.nodes.map(node => ({
    name: node[0],
    value: node[1],
    //symbolSize: node[1] * 10  // 根据节点的权重值设置节点大小
}));

var edges = graphData.edges.map(edge => ({
    source: edge[0],
    target: edge[1],
    value: edge[2]
}));



    //Star数
    var lineChart1 = echarts.init(document.getElementById('lineChart1'));
    lineChart1.setOption( {
      color:["#87cefa","#ff7f50","#32cd32","#da70d6",],
      tooltip : {
           trigger: 'item',
           formatter: "{a}<br/>{b}<br/>{c}个"
       },
       legend: {
        data:data2,
        y: 'bottom',
        x:'center',
        textStyle:{
            color:'#fff',
            fontSize:12
        }
      },
      grid:{
        left: '5%',
        right: '5%',
        bottom: '10%',
        containLabel: true
      },
      calculable : true,
      xAxis : [
          {
              type : 'category',
              boundaryGap : false,
              data : data1,
              axisLine:{
                   lineStyle:{
                       color: '#87cefa'
                   },
               },
               axisLabel : {
                 interval:0,
                 rotate:40,

                   textStyle: {
                       color: '#fff',
                       fontSize:13
                   }
               }
          }
      ],
      yAxis : [
          {
              type : 'value',
              axisLine:{
                  lineStyle:{
                      color: '#87cefa'
                  },
              },
              splitLine: {
                  "show": false
              },
              axisLabel: {
                  textStyle: {
                      color: '#fff'
                  },
                  formatter: function (value) {
                      return value + "个"
                  },
              },
          }
      ],
      series : [
          {
              name:name,
              type:'line',
              smooth:true,
              itemStyle: {normal: {areaStyle: {type: 'default'}}},
              data:data2
          }
      ]

    })

    //fork数
    var lineChart2 = echarts.init(document.getElementById('lineChart2'));
    lineChart2.setOption( {
      color:["#ff7f50","#32cd32","#da70d6",],
      tooltip : {
           trigger: 'item',
           formatter: "{a}<br/>{b}<br/>{c}个"
       },
       legend: {
        data:data4,
        y: 'bottom',
        x:'center',
        textStyle:{
            color:'#fff',
            fontSize:12
        }
      },
      grid:{
        left: '5%',
        right: '5%',
        bottom: '10%',
        containLabel: true
      },
      calculable : true,
      xAxis : [
          {
              type : 'category',
              boundaryGap : false,
              data : data3,
              axisLine:{
                   lineStyle:{
                       color: '#ff7f50'
                   },
               },
               axisLabel : {
                 interval:0,
                 rotate:40,

                   textStyle: {
                       color: '#fff',
                       fontSize:13
                   }
               }
          }
      ],
      yAxis : [
          {
              type : 'value',
              axisLine:{
                  lineStyle:{
                      color: '#ff7f50'
                  },
              },
              splitLine: {
                  "show": false
              },
              axisLabel: {
                  textStyle: {
                      color: '#fff'
                  },
                  formatter: function (value) {
                      return value + "人"
                  },
              },
          }
      ],
      series : [
          {
              name:'厦门中山医院',
              type:'line',
              smooth:true,
              itemStyle: {normal: {areaStyle: {type: 'default'}}},
              data:data4
          }
      ]

    })


    //openrank
    var lineChart3 = echarts.init(document.getElementById('lineChart3'));
    lineChart3.setOption( {
      color:["#32cd32","#da70d6",],
      tooltip : {
           trigger: 'item',
           formatter: "{a}<br/>{b}<br/>{c}个"
       },
       legend: {
        data:data6,
        y: 'bottom',
        x:'center',
        textStyle:{
            color:'#fff',
            fontSize:12
        }
      },
      grid:{
        left: '5%',
        right: '5%',
        bottom: '10%',
        containLabel: true
      },
      calculable : true,
      xAxis : [
          {
              type : 'category',
              boundaryGap : false,
              data : data5,
              axisLine:{
                   lineStyle:{
                       color: '#32cd32'
                   },
               },
               axisLabel : {
                 interval:0,
                 rotate:40,

                   textStyle: {
                       color: '#fff',
                       fontSize:13
                   }
               }
          }
      ],
      yAxis : [
          {
              type : 'value',
              axisLine:{
                  lineStyle:{
                      color: '#32cd32'
                  },
              },
              splitLine: {
                  "show": false
              },
              axisLabel: {
                  textStyle: {
                      color: '#fff'
                  },
                  formatter: function (value) {
                      return value
                  },
              },
          }
      ],
      series : [
          {
              name:'厦门中山医院',
              type:'line',
              smooth:true,
              itemStyle: {normal: {areaStyle: {type: 'default'}}},
              data:data6
          }
      ]

    })

    //Activity
    var lineChart4 = echarts.init(document.getElementById('lineChart4'));
    lineChart4.setOption( {
      color:["#da70d6",],
      tooltip : {
           trigger: 'item',
           formatter: "{a}<br/>{b}<br/>{c}"
       },
       legend: {
        data:data8,
        y: 'bottom',
        x:'center',
        textStyle:{
            color:'#fff',
            fontSize:12
        }
      },
      grid:{
        left: '5%',
        right: '5%',
        bottom: '10%',
        containLabel: true
      },
      calculable : true,
      xAxis : [
          {
              type : 'category',
              boundaryGap : false,
              data : data7,
              axisLine:{
                   lineStyle:{
                       color: '#da70d6'
                   },
               },
               axisLabel : {
                 interval:0,
                 rotate:40,

                   textStyle: {
                       color: '#fff',
                       fontSize:13
                   }
               }
          }
      ],
      yAxis : [
          {
              type : 'value',
              axisLine:{
                  lineStyle:{
                      color: '#da70d6'
                  },
              },
              splitLine: {
                  "show": false
              },
              axisLabel: {
                  textStyle: {
                      color: '#fff'
                  },
                  formatter: function (value) {
                      return value
                  },
              },
          }
      ],
      series : [
          {
              name:'厦门中山医院',
              type:'line',
              smooth:true,
              itemStyle: {normal: {areaStyle: {type: 'default'}}},
              data:data8
          }
      ]

    })

    // 使用 ECharts 创建力导向图
var forceChart1 = echarts.init(document.getElementById('forceChart1'));

forceChart1.setOption({
    series: [
        {
            type: 'graph',
            layout: 'force',
            roam: true,
            emphasis: {
                focus: 'adjacency',
                lineStyle: {
                    width: 5
                }
            },
            nodes: nodes,
            edges: edges
        }
    ]
});


}
