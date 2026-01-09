int getAsciiLine(int i, int x, int y){
if(y==0||x==0)return 0;//since every character has a black border in my texture
if(x==7||y==7) return 0; // culls the bottom and right edge
if(i==0||i==4){
  if(y<=6){
    if(x<=2) return 0;
    if(x<=4) return 1;
    if(x>=5) return 0;
  }
}
if(i==1){
  if(y<=2){
    if(x<=4) return 0;
    if(x<=6) return 1;
    if(x==7) return 0;
  }
  if(y==3){
    if(x<=2) return 0;
    if(x<=5) return 1;
    if(x<=7) return 0;
  }
  if(y==4){
    if(x<=1) return 0;
    if(x<=4) return 1;
    if(x<=7) return 0;
  }
  if(y<=6){
    if(x==0) return 0;
    if(x<=2) return 1;
    if(x<=7) return 0;
  }
}
if(i==2){
  if(y<=2)return 0;
  if(y<=4){
    if(x==0) return 0;
    if(x<=6) return 1;
    if(x==7) return 0;
  }
  if(y<=7)return 0;
}
if(i==3){
  if(y==1){
    if(x==0) return 0;
    if(x<=2) return 1;
    if(x<=7) return 0;
  }
  if(y==2){
    if(x==0) return 0;
    if(x<=2) return 1;
    if(x<=7) return 0;
  }
  if(y==3){
    if(x<=1) return 0;
    if(x<=4) return 1;
    if(x<=7) return 0;
  }
  if(y==4){
    if(x<=2) return 0;
    if(x<=5) return 1;
    if(x<=7) return 0;
  }
  if(y==5){
    if(x<=4) return 0;
    if(x<=6) return 1;
    if(x==7) return 0;
  }
  if(y==6){
    if(x<=4) return 0;
    if(x<=6) return 1;
    if(x==7) return 0;
  }
}
return 0;
}


//this function bellow is used for betting the askii converted color :)
int getAscii(int i, int x, int y){
if(i==0||y==0||x==0)return 0;//since every character has a black border in my texture
if(x==7||y==7) return 0; // culls the bottom and right edge

if(i==1){
  if(y<=5)return 0;
  if(y==6){
    if(x<=2) return 0;
    if(x==3) return 1;
    if(x>=4) return 0;
  }
}
if(i==2){
  if(y==5)return 0;
  if(y<=6){
    if(x<=2) return 0;
    if(x==3) return 1;
    if(x>=4) return 0;
  }
}
if(i==3){
  if(y==1||y==3){
    if(x<=2) return 0;
    if(x<=4) return 1;
    if(x>=5) return 0;
  }
  if(y==2){
    if(x<=1) return 0;
    if(x==2) return 1;
    if(x<=4) return 0;
    if(x==5) return 1;
    if(x>=6) return 0;
  }
  if(y==4||y==6){
    if(x<=2) return 0;
    if(x==3) return 1;
    if(x>=4) return 0;
  }
  if(y==5)return 0;
}
if(i==4){
  if(y<=1||y==7){
    return 0;
  }
  if(y==2||y==6){
    if(x<=2) return 0;
    if(x<=4) return 1;
    if(x>=5) return 0;
  }
  if(y==3||y==5){
    if(x<=1) return 0;
    if(x==2) return 1;
    if(x<=4) return 0;
    if(x==5) return 1;
    if(x>=6) return 0;
  }
  if(y==4){
    if(x<=1) return 0;
    if(x==2) return 1;
    if(x>=3) return 0;
  }
}
if(i==5){
  if(y<=1)return 0;
  if(y==2||y==6){
    if(x<=2) return 0;
    if(x==3) return 1;
    if(x==4) return 1;
    if(x>=5) return 0;
  }
  if(y<=5){
      if(x==2||x==5){return 1;}else return 0;
  }
}
if(i==6){
  if(y==1||y==4){
    if(x<=1) return 0;
    if(x<=4) return 1;
    if(x>=5) return 0;
  }
  if(y<=3){
    if(x<=1) return 0;
    if(x==2) return 1;
    if(x<=4) return 0;
    if(x==5) return 1;
    if(x>=6) return 0;
  }
  if(y<=6){
    if(x<=1) return 0;
    if(x==2) return 1;
    if(x>=3) return 0;
  }
}
if(i==7){
  if(y==1||y==6){
    if(x<=2) return 0;
    if(x==3) return 1;
    if(x==4) return 1;
    if(x>=5) return 0;
  }
  if(y<=5){
    if(x<=1) return 0;
    if(x==2) return 1;
    if(x<=4) return 0;
    if(x==5) return 1;
    if(x>=6) return 0;
  }
}
if(i==8){
  if(y==1){
    if(x<=2) return 0;
    if(x<=5) return 1;
    if(x>=6) return 0;
  }
  if(y==2){
    if(x<=1) return 0;
    if(x==2) return 1;
    if(x<=5) return 0;
    if(x==6) return 1;
    if(x==7) return 0;
  }
  if(y==3){
    if(x<=1) return 0;
    if(x==2) return 1;
    if(x==3) return 0;
    if(x<=6) return 1;
    if(x==7) return 0;
  }
  if(y==4){
    if(x<=1) return 0;
    if(x==2) return 1;
    if(x<=4) return 0;
    if(x==5) return 1;
    if(x>=6) return 0;
  }
  if(y==5){
    if(x<=2) return 0;
    if(x==3) return 1;
    if(x>=4) return 0;
  }
  if(y==6){
    if(x<=3) return 0;
    if(x<=5) return 1;
    if(x>=6) return 0;
  }
}
if (i==9) {
  if (y == 1 || y == 6) {
    if (x >= 2 && x <= 5) return 1;
  }
  if (y >= 2 && y <= 5) {
    if (x >= 1 && x <= 6) return 1;
  }
}
return 0;
}