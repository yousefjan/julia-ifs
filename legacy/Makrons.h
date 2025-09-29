#define NAME 		"3D IFS"
#define RND 		( ( float ) rand ( ) / RAND_MAX )
#define RNDBOOL	( 0 == int ( RND * 2 ) )
#define	RNDSGN	( SGN ( 0.5f + RND ) )

// Z buffer depth:
#define ZDEPTH	16384

// Palette size:
#define PALSIZE	8192	// *** Allways 2 ^ int ( |n| ) ***

// Screen, pixel & Z-buffer sizes:
#define WIDTH		800
#define HEIGHT	600
#define MIDX		( WIDTH>>1 )
#define MIDY		( HEIGHT>>1 )
#define BWIDTH	( WIDTH<<1 )
#define BHEIGHT	( HEIGHT<<1 )
#define BMIDX		( MIDX<<1 )
#define BMIDY		( MIDY<<1 )
#define LWIDTH	4096
#define LHEIGHT	4096
#define LMIDX		( LWIDTH>>1 )
#define LMIDY		( LHEIGHT>>1 )

//// IFS! IFS!! IFS!!! ////
#define ANTAL		8

// Palette displaymodes:
#define SERP		( 1 )
#define ABSZ		( 2 )
#define HOTB		( 3 )
#define JLIA		( 4 )

// Fonts:
#define SMALLFONT		( 0 )
#define MEDIUMFONT	( 1 )
#define BIGFONT			( 2 )

// CODE MAKRONS:

// Draw a filled box:
#define FILLBOX(xstart, ystart, xend, yend, fillcol)\
				(	lixs = xstart,\
        	liys = ystart,\
          lixe = xend,\
          liye = yend,\
          lcol = fillcol,\
          drawBoxi ( )\
        )
// Draw a filled box.

// Outlined textbox: (outline is one point wider than the coordinates)
#define TEXTBOX(xstart, ystart, xend, yend, bordercol, fillcol)\
				(	FILLBOX ( xstart - 1, ystart - 1, xend + 1, yend + 1, bordercol ),\
          FILLBOX ( xstart, ystart, xend, yend, fillcol )\
        )
// Outlined textbox.

// Set a:

#define SETA\
				{ length = sqrtl ( dtx*dtx + dty*dty + dtz*dtz );\
					if ( length > 0.0f )\
    	    {\
     				llength = sqrtl ( dty*dty + dtz*dtz );\
		    	  if ( llength > 0.0f )\
          	{\
					    dty /= llength;\
					    dtz /= llength;\
							t = 1.0f;\
 	    	      if ( dtz < 0.0f )\
	   	        	t = -t;\
  	           cosx = dty;\
				      dty = sqrtl ( ( 1.0f + cosx ) / 2.0f );\
				      dtz = t * sqrtl ( ( 1.0f - cosx ) / 2.0f );\
					    dty *= llength;\
					    dtz *= llength;\
  	      	}\
    	      llength = sqrtl ( dtx*dtx + dty*dty );\
      	    if ( llength > 0.0f )\
        	  {\
					    dtx /= llength;\
					    dty /= llength;\
				      t = 1.0f;\
      	      if ( dty < 0.0f )\
	      	    	t = -t;\
			    	  cosx = dtx;\
			      	dtx = sqrtl ( ( 1.0f + cosx ) / 2.0f );\
				      dty = t * sqrtl ( ( 1.0f - cosx ) / 2.0f );\
					    dtx *= llength;\
					    dty *= llength;\
				    }\
        	  llength = sqrtl ( dtx*dtx + dty*dty + dtz*dtz );\
          	length = sqrtl ( length );\
	  	      if ( llength > 0.0f )\
  	  	    {\
						  dtx = ( dtx / llength ) * length;\
						  dty = ( dty / llength ) * length;\
						  dtz = ( dtz / llength ) * length;\
          	}\
					}\
        }
// Set a.

// Set b:
#define SETB\
				{	length = sqrtl ( dtx*dtx + dty*dty + dtz*dtz );\
					if ( length > 0.0f )\
    	    {\
     				llength = sqrtl ( dty*dty + dtz*dtz );\
		    	  if ( llength > 0.0f )\
          	{\
							t = 1.0f;\
 		          if ( dtz < 0.0f )\
   		        	t = -t;\
      	       cosx = dty / llength;\
			  	    dty = sqrtl ( ( 1.0f + cosx ) / 2.0f );\
			    	  dtz = t * sqrtl ( ( 1.0f - cosx ) / 2.0f );\
	        	}\
  	        llength = sqrtl ( dtx*dtx + dty*dty );\
    	      if ( llength > 0.0f )\
      	    {\
			  	    t = 1.0f;\
          	  if ( dty < 0.0f )\
	          		t = -t;\
	            cosx = dtx / llength;\
				      dtx = sqrtl ( ( 1.0f + cosx ) / 2.0f );\
				      dty = t * sqrtl ( ( 1.0f - cosx ) / 2.0f );\
				    }\
        	  llength = sqrtl ( dtx*dtx + dty*dty + dtz*dtz );\
          	length = sqrtl ( length );\
	  	      if ( llength > 0.0f )\
  	  	    {\
						  dtx = ( dtx / llength ) * length;\
						  dty = ( dty / llength ) * length;\
						  dtz = ( dtz / llength ) * length;\
          	}\
					}\
        }
// Set b.

// Set c:
#define SETC\
				{	length = sqrtl ( dtx*dtx + dty*dty + dtz*dtz );\
					if ( length > 0.0f )\
    	    {\
						llength = sqrtl ( dty*dty + dtz*dtz );\
  	  			if ( llength > 0.0f )\
			    	{\
	    				yt = dty / llength;\
	  	  			zt = -dtz / llength;\
							dty = length;\
							dtz = 0.0f;\
	    	      llength = sqrtl ( dtx*dtx + dty*dty );\
  	    	    if ( llength > 0.0f )\
    	    	  {\
				    	  t = 1.0f;\
        	    	if ( dty < 0.0f )\
	        	  		t = -t;\
	            	cosx = dtx / llength;\
  	            llength = sqrtl ( llength );\
					      dtx = sqrtl ( ( 1.0f + cosx ) / 2.0f ) / llength;\
					      dty = t * sqrtl ( ( 1.0f - cosx ) / 2.0f ) / llength;\
					    }\
        		  llength = sqrtl ( yt*yt + zt*zt );\
          		if ( llength > 0.0f )\
		          {\
					      t = 1.0f;\
    		        if ( zt < 0.0f )\
	    		      	t = -t;\
        		    cosx = yt / llength;\
			    		  yt = sqrtl ( ( 1.0f + cosx ) / 2.0f );\
			      		zt = t * sqrtl ( ( 1.0f - cosx ) / 2.0f );\
					    }\
							llength = sqrtl ( yt*yt + zt*zt );\
  		  			if ( llength > 0.0f )\
				    	{\
    						yt = yt / llength;\
    						zt = -zt / llength;\
	        		}\
							dtz = zt * dty;\
							dty = yt * dty;\
    	      }\
      	    llength = sqrtl ( dtx*dtx + dty*dty + dtz*dtz );\
        	  length = sqrtl ( length );\
  	      	if ( llength > 0.0f )\
	    	    {\
						  dtx = ( dtx / llength ) * length;\
						  dty = ( dty / llength ) * length;\
						  dtz = ( dtz / llength ) * length;\
        	  }\
	        }\
        }
// Set c.

// Set d:
#define SETD\
				{	length = sqrtl ( dtx*dtx + dty*dty + dtz*dtz );\
					if ( length > 0.0f )\
   	   	  {\
	  	      amp = sqrtl ( length );\
						if ( length == fabsl ( dtx ) )\
    	   	  {\
      		  	if ( dtx < 0.0f )\
        		  {\
          		  angle = RND * pii;\
            		dty = cosl ( angle ) * amp;\
	            	dtz = sinl ( angle ) * amp;\
	  	      		dtx = 0.0f;\
  	  	      }\
    	  	    else\
      	  			dtx = amp;\
  	    	  }\
        		else if ( length > 0.0f )\
	  	      {\
  	  	    	dtx = ( ( dtx - length ) / 2.0f ) + length;\
    	  	  	dty = dty / 2.0f;\
      	  		dtz = dtz / 2.0f;\
        	  	length = amp / sqrtl ( dtx*dtx + dty*dty + dtz*dtz );\
						  dtx *= length;\
					  	dty *= length;\
						  dtz *= length;\
		        }\
	        }\
        }
// Set d.

// Set e:
#define SETE\
				{	length = sqrtl ( dtx*dtx + dty*dty + dtz*dtz );\
					if ( length > 0.0f )\
   	   	  {\
	  	      amp = powl ( length, 1.0f / 3.0f );\
						if ( length == fabsl ( dtx ) )\
    	   	  {\
      		  	if ( dtx < 0.0f )\
        		  {\
          		  angle = RND * pii;\
            		dty = cosl ( angle ) * amp;\
	            	dtz = sinl ( angle ) * amp;\
	  	      		dtx = 0.0f;\
  	  	      }\
    	  	    else\
      	  			dtx = amp;\
  	    	  }\
        		else if ( length > 0.0f )\
	    	    {\
  		        length = sqrtl ( dty*dty + dtz*dtz );\
    	        if ( length > 0.0f )\
      	  	  {\
        	    	y = dty / length;\
          	  	z = dtz / length;\
    	      	}\
							else\
  	      	  {\
    	        	y = 1.0f;\
      	      	z = 0.0f;\
    	  	    }\
          	 	dty = length;\
  	        	length = sqrtl ( dtx*dtx + dty*dty );\
	    	      if ( length > 0.0f )\
  	    	    {\
				  	    t = 1.0f;\
      	    	  if ( dty < 0.0f )\
	      	    		t = -t;\
	        	    cosx = dtx / length;\
				    	  dtx = sqrtl ( ( 1.0f + cosx ) / 2.0f );\
				      	dty = t * sqrtl ( ( 1.0f - cosx ) / 2.0f );\
					    }\
  	          dtz = dty * z;\
    	        dty = dty * y;\
      	    	length = amp / sqrtl ( dtx*dtx + dty*dty + dtz*dtz );\
						  dtx *= length;\
						  dty *= length;\
					  	dtz *= length;\
						}\
					}\
        }
// Set e.

// Set d3:
#define SETD3\
				{	length = sqrtl ( dtx*dtx + dty*dty + dtz*dtz );\
					if ( length > 0.0f )\
   	   	  {\
	  	      amp = powl ( length, ( 1.0f / 3.0f ) );\
						if ( length == fabsl ( dtx ) )\
    	   	  {\
      		  	if ( dtx < 0.0f )\
        		  {\
          	  angle = RND * pii;\
            	dty = cosl ( angle ) * amp;\
	            dtz = sinl ( angle ) * amp;\
  	      		dtx = 0.0f;\
    	      	}\
      	    	else\
        				dtx = amp;\
  	      	}\
	        	else if ( length > 0.0f )\
  		      {\
    		    	dtx = ( ( dtx - length ) * (2.0f * 3.0f) ) + length;\
      		  	dty = dty * (2.0f * 3.0f);\
	        		dtz = dtz * (2.0f * 3.0f);\
  	        	length = amp / sqrtl ( dtx*dtx + dty*dty + dtz*dtz );\
						  dtx *= length;\
						  dty *= length;\
						  dtz *= length;\
	        	}\
        	}\
        }
// Set d3.

// Set 2D:
#define SET2D\
				{	dtz = 0.0f;\
  	      length = sqrtl ( dtx*dtx + dty*dty );\
					if ( length > 0.0f )\
   	   	  {\
	    	    amp = sqrtl ( length );\
						if ( length == fabsl ( dtx ) )\
    	      {\
      	  		if ( dtx < 0.0f )\
        	  	{\
        				dty = amp;\
	        			dtx = 0.0f;\
	  	        }\
  	  	      else\
    	  	  		dtx = amp;\
      	    }\
	        	else if ( length > 0.0f )\
		        {\
  		      	dtx = ( ( dtx - length ) / 2.0f ) + length;\
  	  	    	dty = dty / 2.0f;\
	      	    length = amp / sqrtl ( dtx*dtx + dty*dty );\
						  dtx *= length;\
						  dty *= length;\
	          }\
          }\
        }
// Set 2D.

// Set 2D3:
#define SET2D3\
				{ dtz =.0f;\
        	length = sqrtl ( dtx*dtx + dty*dty );\
					if ( length > 0.0f )\
   	   	  {\
	  	      amp = powl ( length, ( 1.0f / 3.0f ) );\
						if ( length == fabsl ( dtx ) )\
    	   	  {\
      		  	if ( dtx < 0.0f )\
        		  {\
          	  	dty = amp;\
  	      			dtx = 0.0f;\
	    	      }\
  	    	    else\
    	    			dtx = amp;\
  	  	    }\
        		else if ( length > 0.0f )\
  	      	{\
	    	    	dtx = ( ( dtx - length ) / 3.0f ) + length;\
  	    	  	dty = dty / 3.0f;\
    	      	length = amp / sqrtl ( dtx*dtx + dty*dty );\
						  dtx *= length;\
						  dty *= length;\
	        	}\
        	}\
        }
// Set 2D3.

// 2X:
#define MOD2X\
				{	pmodi = 0;\
        	if ( duoi )\
        	{\
            dtx = - dtx;\
            dty = - dty;\
            dtz = - dtz;\
            palupflag = true;\
        	}\
        }
// 2X.

// 3X:
#define MOD3X\
				{	pmodi = 0;\
          length = sqrtl ( dty*dty + dtz*dtz );\
          if ( length > 0.0f )\
          {\
          	y = dty / length;\
          	z = dtz / length;\
    	    }\
					else\
					{\
 	        	y = 1.0f;\
   	      	z = 0.0f;\
 	  	    }\
 	        dty = length;\
          di = int ( RND * 3 );\
          coli = ( di << 1 ) + 4;\
          if ( di == 0 )\
	          palupflag = true;\
          if ( di == 1 )\
          {\
					  t = r3Xx * dtx - r3Xy * dty;\
					  dty = r3Xx * dty + r3Xy * dtx;\
			  		dtx = t;\
         	}\
          if ( di == 2 )\
          {\
					  t = r3Xx * dtx - (  -r3Xy ) * dty;\
					  dty = r3Xx * dty + ( -r3Xy ) * dtx;\
			  		dtx = t;\
         	}\
          dtz = dty * z;\
          dty = dty * y;\
        }
// 3X.

// 4X:
#define MOD4X\
				{	pmodi = 0;\
          if ( duoi )\
          {\
          	t = -dty;\
            dty = dtx;\
            dtx = t;\
            palupflag = true;\
          }\
        }
// 4X.

// 6X:
#define MOD6X\
				{	pmodi = 0;\
          if ( duoi )\
          {\
          	t = -dty;\
            dty = dtz;\
            dtz = dtx;\
            dtx = t;\
            palupflag = true;\
          }\
        }
// 6X.

// 6XX:
#define MOD6XX\
				{	pmodi = 1;\
          if ( multi > 6 )\
          	multi = 0;\
          coli = multi + 4;\
          if ( multi )\
          {\
	          for ( di = 0; di < multi; di++)\
  	        {\
    	      	t = -dty;\
      	      dty = dtz;\
        	    dtz = dtx;\
          	  dtx = t;\
	          }\
          }\
        }
// 6XX.

// 8X:
#define MOD8X\
				{	pmodi = 1;\
          multi &= 0x07;\
          coli = multi + 4;\
          if ( multi & 0x04 )\
          	dtx = - dtx;\
          if ( multi & 0x02 )\
          	dty = - dty;\
          if ( multi & 0x01 )\
          	dtz = - dtz;\
        }
// 8X.

// 2X + 6X:
#define MOD2X6X\
				{	pmodi = 0;\
          if ( int ( RND * 2 ) )\
          {\
          	t = -dty;\
            dty = dtz;\
            dtz = dtx;\
            dtx = t;\
            palupflag = true;\
          }\
        	if ( duoi )\
        	{\
            dtx = - dtx;\
            dty = - dty;\
            dtz = - dtz;\
            palupflag = true;\
        	}\
        }
// 2X + 6X.

/* 50/50 mode - disabled - using 25/75 fom edition seventeen.
// Color cycler:
#define COLPAL\
				{	tcolor = lpCols [ pali ];\
				  tRed = ( tcolor >> 16 ) & 0xFF;\
				  tGreen = ( tcolor >> 8 ) & 0xFF;\
				  tBlue = tcolor & 0xFF;\
				  dcr = ( ( dcr + tRed) >> 1 ) & 0xFF;\
				  dcg = ( ( dcg + tGreen) >> 1 ) & 0xFF;\
				  dcb = ( ( dcb + tBlue) >> 1 ) & 0xFF;\
        }
// Color cycler.
*/

// Color cycler:
#define COLPAL\
				{	tcolor = lpCols [ pali ];\
				  tRed = ( tcolor >> 16 ) & 0xFF;\
				  tGreen = ( tcolor >> 8 ) & 0xFF;\
				  tBlue = tcolor & 0xFF;\
				  dcr = ( ( dcr + ( tRed * 3 ) )   >> 2 ) & 0xFF;\
				  dcg = ( ( dcg + ( tGreen * 3 ) ) >> 2 ) & 0xFF;\
				  dcb = ( ( dcb + ( tBlue * 3 ) )  >> 2 ) & 0xFF;\
        }
// Color cycler.

// Color selector:
#define COLMOD\
				{	dcr = ( ( dcr + tcr [ coli ] ) >> 1 ) & 0xFF;\
					dcg = ( ( dcg + tcg [ coli ] ) >> 1 ) & 0xFF;\
					dcb = ( ( dcb + tcb [ coli ] ) >> 1 ) & 0xFF;\
        }
// Color selector.
