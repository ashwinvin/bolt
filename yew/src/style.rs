use stylist::css;

pub fn get_styles() -> stylist::StyleSource {
    return css!(
        r#"
        * {
	margin: 0;
}

body {
	cursor: default;
	user-select: none;
	color: white;
	font-family: Arial, Helvetica, sans-serif;
}

body {
	display: flex;
	flex-direction: column;
	height: 100vh;
	width: 100vw;
	overflow: hidden;
}

.main {
	display: flex;
	flex-direction: row;
}

.navbar {
	height: 5vh;
	display: flex;
	justify-content: space-between;
	align-items: center;
	background-color: rgb(4, 12, 23);
	padding: 0 20px;
	border-bottom: 0.5px solid gray;
}

.sidebar1 {
	width: 5vw;
	background-color: rgb(4, 12, 23);
	height: 90vh;
	border-right: 0.5px solid gray;
}

.sidebar2 {
	width: 15vw;
	background-color: rgb(4, 12, 23);
	height: 90vh;
	border-right: 0.5px solid gray;
}

.sidebaritem {
	display: flex;
	flex-direction: column;
	align-items: center;
	justify-content: center;
	background: rgb(7, 31, 56);
	height: 50px;
	font-size: 10px;
	margin-bottom: 5px;
	margin: 3px;
}

.sidebaritem:hover {
	 background: rgb(23, 59, 97);
}

.sidebaritem-selected {
	border-left: 2px solid orange;
	background: rgb(23, 59, 97);
}


.sidebar2item {
	display: flex;
	flex-direction: column;
	align-items: left;
	justify-content: center;
	background: rgb(7, 31, 56);
	height: 20px;
	font-size: 10px;
}

.sidebar2item:hover {
	 background: rgb(23, 59, 97);
}

.sidebar2item-selected {
	background: rgb(23, 59, 97);
	border-left: 2px solid orange;
}

.content {
	height: 90vh;
	width: 80vw;
	background-color: rgb(3, 7, 13);
	display: flex;
	flex-direction: column;
	align-items: left;
}

.requestbar {
	display: flex;
	flex-direction: row;
	align-items: center;
	margin-left: 10px;
	margin-top: 20px;
	margin-bottom: 20px;
}

.console {
	height: 5vh;
	display: flex;
	flex-direction: column;
	background-color: rgb(4, 12, 23);
	border-top: 0.5px solid gray;
}

.logo {
	display: flex;
	flex-direction: row;
	align-items: center;
	justify-content: center;
	font-size: 25px;
	font-family: Arial, Helvetica, sans-serif;
}

.nav-links {
	display: flex;
	flex-direction: row;
}

.nav-links div {
	margin-right: 20px;
}

.pointer {
	cursor: pointer;
}



.methodselect {
	width: 120px;
	height: 40px;
  
  border: 0.5px solid gray;
	color: white;
	font-size: 15px;
 
  -webkit-appearance: none;
  -moz-appearance: none;

	background: rgb(23, 59, 97);
	border-radius: 0px;

	display: flex;
	flex-direction: row;
	align-items: center;
	justify-content: center;
}

.urlinput {
	height: 40px;
	width: 60vw;
	background: rgb(23, 59, 97);
	color: white;
	font-size: 15px;
	border: 0.5px solid gray;
}

.urlinput:focus {
  outline: none;
}

.sendbtn {
	height: 40px;
	width: 70px;
	background: rgb(171, 113, 7);
	border: 0px;
	color: white;
	margin-left: 5px;
	font-size: 15px;
}

.sendbtn:hover {
	 background: rgb(186, 123, 6);
}

.req, .resp {
	height: 100%;
}

.req .tabcontent {
	height: 100%;
}


.resp .tabcontent {
	height: 100%;
}

.respbody {
	width: 100%;
	height: 100%;
	background-color: rgb(3, 7, 13);
	border: 0.5px solid gray;
	color: white;
	font-size: 15px;
}

.respbody:focus {
  outline: none;
}


.reqbody {
	width: 100%;
	height: 70%;
	background-color: rgb(3, 7, 13);
	border: 0.5px solid gray;
	color: white;
	font-size: 15px;
}

.reqbody:focus {
  outline: none;
}

.reqtabs {
  display: flex;
	flex-direction: row;
  width: 100%;
	margin-left: 10px;
}

.resptabs {
  display: flex;
	flex-direction: row;
  width: 100%;
 	
}

.tab {
  display: flex;
	flex-direction: row;
	align-items: center;
	justify-content: center;
	
	height: 30px;
	width: 70px;
}

.tab:hover {
	 background: rgb(23, 59, 97);
}

.tabSelected {
  background: rgb(23, 59, 97);
}


.respline {
	display: flex;
	flex-direction: row;
	align-items: center;
	justify-content: space-between;
}

.respstats {
	display: flex;
	flex-direction: row;
}

.respstat {
	font-size: 13px;
	min-width: 90px;
	margin-right: 10px;
}


table {
  border-collapse: collapse;
  width: 100%;
}

th, td {
  text-align: left;
  padding: 8px;
}

th {
  border: 0.5px solid #ddd;
}

td {
  border: 0.5px solid #ddd;
}

        
        
        "#
    );
}
