/* --------------------------------------------------------------------- *
										 	 ___								 		 _
											// \\  	 _   |	 _	 	 	 \\
									   //	 	 		 \\  |	//		 	 //
									  //		  		\\___//		 	 	//
						  			\\____	 		//   \\   	 //
										   	 \\ ___//		  \\___	//
													\\	 \\		  //   //
									 _      //	  \\___//   //
									 \\    //	  	//   \\  //			_
									  \\__//	 	 //  |  \\ \\____//
									   \--/			/    |  	\ \----/

 * --------------------------------------------------------------------- *
 		SoL Developments - SunCode -- 3D RJIFS -- v 0.17.54 SoL 2005.

		start-ed 11-02-05
		last-ed 20-04-05

 (I'm using tab-size = 2 chars and font = Courier new 13 pixels)
 * --------------------------------------------------------------------- *
   /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
/* --------------------------------------------------------------------- *
			MAKRONS
 * --------------------------------------------------------------------- */
#include "makrons.h"
/* -------------------------------------------------------------------- *
			INCLUDES
 * --------------------------------------------------------------------- */
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <windowsx.h>
#include <ddraw.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <math.h>
#include <mem.h>
#include <time.h>
/* --------------------------------------------------------------------- *
			DECLARE GLOBAL VARIABLES:
 * --------------------------------------------------------------------- */
// Use Widows API and DirectX referenses for information
// about the following variable declarations.
  DDSCAPS             		ddscaps;
  DDSURFACEDESC       		ddsd;
  HDC                 		hdc;
  HRESULT             		ddrval;
	HWND                		hwnd;
	LPDIRECTDRAW            lpDD;
	LPDIRECTDRAWSURFACE     lpDDSBuffer;
	MSG	msg;
  WNDCLASS            		wc;

	// A number of "always useful" constants:
  const double	pi = 3.141592654f;		// Precission enough?

  	// These 'constants' are initiated in the function "doInit":
  	double			rad;		// A radian.
  	double			pii;    // 2 x pi.
  	double			phi;    // The golden ratio.

	// A number of "always useful" variables:
	bool		tflag;			// A temporary flag.
  int  		tmp;				// A temporary integer number.
  double	temp;       // A temporary floating point number.
  double	angle;			// Used to store an angle, (radians or degrees)
  double	cosx;				// Cos of an angle.
  double	siny;       // Sin of an angle.
  double	amp;				// The amplitude of ...
  double	length;			// The length of ...
  double	llength;		// If two lengths of ... is needed.

	// Temporary index variables:
  int			index;
  int			i;
  int			j;
  int			xi;
  int			yi;

	// Temporary n-counters:
  int			n;
  int			p;
  int			r;

  // Four dimensions:

		// Variable + temp dito:
		double	x, tmpx;
		double	y, tmpy;
		double	z, tmpz;
		double	t, tmpt;				// Also used for 'temp'.

		// Temp storage if a square is used more than once:
		double	xx;
		double	yy;
		double	zz;
		double	tt;

		// Constant:
  	double	a;
  	double	b;
  	double	c;
  	double	d;

/* --------------------------------------------------------------------- *
		Fractal renderer variablel declarations:

    Notes:

    	Below a mess!, (well it's getting better), but this section
      will be cleaner & get better comments in the near future.
      To download the latest version of this source-code, go to:

      				"http://members.chello.se/solgrop/3djulia.htm"

  		Some of this stuff is not used here, this code is
  		originaly my 3D linear IFS renderer.

 * --------------------------------------------------------------------- */
// Main task flags:
  bool		runflag = false;
  bool		renderactive = false;
	int			programMode = 1;

// Image buffers & stuff:
  long 	* lpBuf;											 	// Pointer to screen buffer.
  long		lk;													 	// Size of screen line, (y coordinate incrementor).
  long 		lpCols [ PALSIZE ];					 	// Buffer for colour palette.
  long		pict [ BHEIGHT ] [ BWIDTH ]; 	// Buffer for picture.
  int			bpict [ BHEIGHT ] [ BWIDTH ];	// Z-buffer for picture, max value = ZDEPTH.
  int			light [ LHEIGHT ] [ LWIDTH ];	// Z-buffer for shadows, max value = ZDEPTH.

	// Used for writing lines:
  int			lixs, liys, lixe, liye;
  long		lcol;
  double	lxs, lys, lxe, lye;
  double	RATIO, RERAT;

	// Variable to store the background colour -
  //(initiate to default):
  long		bgcolor = 0x00103050;

	// Flag if a new palette is requested:
  bool		NewPalette = false;

	// Flag if a new set is to be rendered:
  bool		newset;

	// Variable to store the current background mode:
  int		showbackground;

  // Rect that containas the whole screen:
	RECT		rfsc = { 0, 0, WIDTH, HEIGHT };
  // Rect used for anything needed:
	RECT		tbox = { 0, 0, 0, 0 };

	// Fonts, three diffrent used here:
  HFONT		smallfont, mediumfont, bigfont;

	// Background mode, text, colours and text colours:
	char	*	textbgmess [ 5 ] = { "On", "Off blue", "Off black", "Off grey", "Off white" };
  // note: RGB
  long 		bgcol [ 5 ] = { 0x00103050, 0x00262332, 0x00000000, 0x00808080, 0x00FFFFFF };
  // note: BGR
  long 		txcol [ 5 ] = { 0x0040FFFF, 0x0080FFFF, 0x00FFFFFF, 0x00000000, 0x00800000 };

	// Light model texts:
	char	*	textlight [ 2 ] = { "Dark", "Light" };
	char	*	textpales [ 3 ] = { "Normal", "Flourescent", "Filament" };

	// Fractal set texts:
	char	* settexts [ 8 ] = { "Set a", "Set b", "Set c", "Set d", "Set e", "Set d3", "2D", "2D3" };

	// X-mode texts & x coordinates for output:
	char	* xmodtexts [ 7 ] = { "2X", "3X", "4X", "6X", "6XX", "8X", "2X6X" };
  int			xmodcoords [ 7 ] = { 760, 760, 760, 760, 751, 760, 745 };

	// Temporary string buffer:
  char		stringbuf [ 256 ];

/* ---------------------------------------------------------------------
		VARIABLES FOR THE ITERATION-LOOP:
   --------------------------------------------------------------------- */
	// Image scale ratios, (pic & shadows):
	double	ims = 2500, lims = 5000, size;

	// Zoomfactor and zoom in/out factors:
  double	imszoom = 1.0f, zoomup = 1.05946, zoomdown = 0.94387;

	// Camera translation:
  //(not used at the moment but in the code)
	double	CPOSX =	0.0f;
	double	CPOSY =	0.0f;
	double	CPOSZ =	0.0f;

	// Physical screen coordinates, (x, y & z-buffer) + temp dito:
  int			nX, nXt;
  int			nY, nYt;
  int			nZ, nZt;

  //// Light! ////

		// Flag if pixel is written to the shadows map:
  	bool	 	doshadow;

		// Light models:
		int		whitershade = 0;
		int 	lightness = 0;

		// Various temp variables used to calculate lights:
  	int			ncols, bright, blight, tbright, overexpose, toverexpose;
  	double	minbright, maxbright, luma, tluma;

		// Light rotation angles & cos + sin for these:
  	double	lrxx, lrxy, lrxv;
  	double	lryx, lryy, lryv;

		// Attractor-glow!
	  bool 		useglow;
 		double	glow = 1.0f, largel = 0.0001f;
 		double	bglow = 1.0f, blargel = 0.0001f;
  	double	dglow = 1.0f, dlargel = 0.0001f;

	// Palette index:
  int 		pali = 0;
	// "Second root" palette index:
  int 		pali2 = 0;

  // Temp storage for colour, (used for "second root").
  int	tcolr, tcolg, tcolb;

	// Rotator, (cos, sin, angle):
  double	rxx, rxy, rxv;
  double	ryx, ryy, ryv;

  //// IFS! IFS!! IFS!!! ////
  long		itersdone = 0;				// Number of iterations done so far.
  long		pixelswritten = 0;		// Number of pixels written to the image Z-buffer.
  long		spixelswritten;				// Storage for number of pixels written.
  long		shadowswritten = 0;		// Number of pixels written to the shadow map Z-buffer.
  long		sshadowswritten;			// Storage for number of shadows written.
  int			pti;									// Index counter for IFS loop.
  double	btx, bty, btz; 				// Bottom plane 3D point.
  double	dtx, dty, dtz;				// 3D point - the fractal.
  double	ltx, lty, ltz;        // 3D point Light position.
  double	xt, yt, zt;        		// Pixel position in scene.
  double	ntx, nty, ntz;     		// Normal of pixel (if needed).
  double	nxt, nyt, nzt;        // Pixel normal position in scene.

	// ISF random index, ("background" and "dragon"):
  int			bi, di;

	// Translators for IFS:
  //( + 4 is the background 4 point space fill square)
  double	tx [ ANTAL + 4 ];
  double	ty [ ANTAL + 4 ];
	double	tz [ ANTAL + 4 ];

  // Scale factors for IFS:
	double	sc [ ANTAL + 4 ];

	// Rotators for IFS, (cos, sin, angle):
  double	drxx, drxy, drxv;
  double	dryx, dryy, dryv;
  double	drzx, drzy, drzv;

	// Colours for IFS:
	unsigned char	tcr [ ANTAL + 4 ];
	unsigned char	tcg [ ANTAL + 4 ];
	unsigned char	tcb [ ANTAL + 4 ];

	// Various variables used to store R, G, B values:
  int	bcr, bcg, bcb, dcr, dcg, dcb, crt, cgt, cbt;
  int tRed, tGreen, tBlue;

	// Various variables used to store xRGB values:
  long int color, tcolor, bcolor, dcolor;

  // Number of colours used:
  int	nCols;

  // ** JULIAS ** //

	// index trix!
  long 		indxs = 0;
  long 		indxuse;
  int   	indxn;
  double  probability;
	// Index for repetitions:
  int			repti;
  int			maxrepti;
  // Swap flags:
  bool 		useswap;
  bool		swapflag;

	// Flag for palette index move direction (up & down possible):
  bool		palupflag;

	// Used for the (not so good) secret ingredient part =)
  bool		secretingredient;
  bool		secretsquare;
	bool		secretextracoord;
  int			secretsize;

	// Global scale ratio:
  //(used to fit the fractal into the scene)
  double	pixscale;

  // Index for preset:
  int 		ui = 9;
	// index for set:
  int			seti = 3;
	// Index for X-mode:
  int			nxi = 0;
	// Ind	ex for colour:
  int		coli;
	// Index for palette mode:
  int			pmodi;

	// 50/50 ratio random index & storage:
  bool		duoi;
  bool		sduoi;
	// x/y ratio random index & storage:
  int			multi;
  int			smulti;

	// Rotators for 3X, (cos, sin, angle):
  double	r3Xx, r3Xy, r3Xv;

	// Storage for preset coordinates:
  double	pcx [ 10 ];
  double	pcy [ 10 ];
  double	pcz [ 10 ];

	// Parameter positions saved here:
  double	xbuf [ 10 ];
  double	ybuf [ 10 ];
  double	zbuf [ 10 ];

/* --------------------------------------------------------------------- *
			DECLARE FUNCTIONS
 * --------------------------------------------------------------------- */
void CamAng ( void );
void clearallbufs (long RGBdata );
void clearscreen ( long RGBdata );
void clearscreenbufs ( long RGBdata );
void CreatePalette ( void );
static BOOL doInit ( HINSTANCE hInstance, int nCmdShow );
void DoMyStuff ( void );
void drawBox ( void );
void drawBoxi ( void );
void drawLine ( void );
void drawMulticolLine ( void );
static void finiObjects ( void );
void getspot ( void );
void IFSlight ( void );
void IFSplot ( void );
void initiateIFS ( void );
void initiatetext ( void );
void LitAng ( void );
void manual ( void );
void newrender ( void );
void newsetup ( void );
void pixelsmess ( void );
void printcoordinfo ( void );
void printsceneinfo ( void );
void printsetinfo ( void );
void rotatelight ( void );
void rotateview ( void );
int SGN ( double x );
void ShowPalette ( int mode );
void showpic ( void );
void spacemess ( void );
void SunCode ( void );
void textline ( int curposx, int curposy, char * stringdata, int fontindex, long textcolor );
void unrotatelight ( void );
void unrotateview ( void );
/* --------------------------------------------------------------------- *
			Return sign of x:
 * --------------------------------------------------------------------- */
int SGN ( double x )
{
	if ( x < 0.0f )
  	return ( -1 );
  else
  	return ( 1 );
} // SGN.
/* --------------------------------------------------------------------- *
			Windows functions:

      First WinMain:

 * --------------------------------------------------------------------- */
int PASCAL WinMain ( HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow )
{
	lpCmdLine = lpCmdLine;
	hPrevInstance = hPrevInstance;
	if ( ! doInit ( hInstance, nCmdShow ) )
		return ( false );

  // Setup fonts:
	initiatetext ( );

	// Setup fractal render functions:
  initiateIFS ( );

	// *********************
 	// *     MAIN-LOOP     *
 	// *********************
	runflag = true;
	// here we go:
  while ( runflag )
  {
	  if ( PeekMessage ( &msg, NULL, 0, 0, PM_REMOVE ) )
		{
      if ( msg.message == WM_QUIT )
      	runflag = false;

			TranslateMessage ( &msg );
			DispatchMessage ( &msg );
  	}
		// if render screen is up then iterate:
	  if ( ( programMode == 0 ) && renderactive )
			DoMyStuff ( );
  }
 	// *********************
	// * End of MAIN-LOOP. *
  // *********************

	finiObjects ( );
	DestroyWindow ( hwnd );

	return ( msg.wParam );
}// End of program.
/* --------------------------------------------------------------------- *
			Window proc:
 * --------------------------------------------------------------------- */
long FAR PASCAL WindowProc ( HWND hWnd, UINT message, WPARAM wParam, LPARAM lParam )
{
	if ( ! runflag )
  	return DefWindowProc ( hWnd, message, wParam, lParam );

	switch ( message )
  {
    case WM_CREATE:
    case WM_PAINT:
			switch ( programMode )
  	  {
      	case 0:
					// Copy & ant-anilize from pixel-buffer to screen:
          showpic ( );
					// Initiate iteration parameters:
					newrender ( );
					renderactive = true;
					// Clear screen if set was changed?:
          if ( newset )
          {
          	clearallbufs ( bgcol [ showbackground ] );
          	newset = false;
						// Then - agin! =)
          	showpic ( );
          }
					// "tag" screen =)
					SunCode ( );
				break;
	    	case 1:
					// View info screen:
					manual ( );
				break;
	    } // programMode.
		break;

    case WM_SETCURSOR:
			SetCursor ( NULL );
		break;

    case WM_KEYUP:
			// Functions only active if render screen:
			if ( renderactive )
	    {
  	  	switch ( wParam )
    	  {
      		case VK_UP:
      		case VK_RIGHT:
	      	case VK_DOWN:
  	    	case VK_LEFT:
						clearscreenbufs ( bgcol [ showbackground ] );
						clearscreen ( bgcol [ showbackground ] );
						SunCode ( );
	        break; // Arrow keys.
  	    }
    	} // ? renderactive.
    break; // WM_KEYUP.

    case WM_KEYDOWN:
    	// Do keys, (two modes)
			switch ( programMode )
  	  {
      	case 0:
        	// Render screen:
					switch ( wParam )
					{
						case VK_ESCAPE:
							renderactive = false;
			    		PostMessage ( hWnd, WM_CLOSE, 0, 0 );
			    	break; // Escape.

						case VK_SPACE:
							renderactive = false;
							programMode = 1;
			    		PostMessage ( hWnd, WM_PAINT, 0, 0 );
		    		break; // Space.

						case VK_PRIOR:
							imszoom *= zoomup;
							clearallbufs ( bgcol [ showbackground ] );
							clearscreen ( bgcol [ showbackground ] );
							SunCode ( );
		    		break; // page up.

						case VK_NEXT:
							imszoom *= zoomdown;
							clearallbufs ( bgcol [ showbackground ] );
							clearscreen ( bgcol [ showbackground ] );
							SunCode ( );
		    		break; // page down.

						case VK_HOME:
							ryv = 0.0f * rad;
							rxv = 0.0f * rad;
					  	CamAng ( );
							imszoom = 1.0f;
							newrender ( );
							newrender ( );
							clearallbufs ( bgcol [ showbackground ] );
							clearscreen ( bgcol [ showbackground ] );
							SunCode ( );
		    		break; // page down.

    	  		case VK_UP:
							rxv += 0.01f;
              if ( int ( rxv ) > 180 )
              	rxv = 180.0f;
						  CamAng ( );
            break; // Up key.

		     		case VK_RIGHT:
							ryv += 0.01f;
              if ( int ( ryv ) > 180 )
              	ryv = 180.0f;
						  CamAng ( );
            break; // Right key.

	      		case VK_DOWN:
							rxv -= 0.01f;
              if ( int ( rxv ) < -180 )
              	rxv = -180.0f;
						  CamAng ( );
            break; // Down key.

  	    		case VK_LEFT:
							ryv -= 0.01f;
              if ( int ( ryv ) < -180 )
              	ryv = -180.0f;
						  CamAng ( );
            break; // Left key.

						case 'A':
							ryv = -180.0f * rad + RND * 360.0f * rad;
					 		ryx = cosl ( ryv );
							ryy = sinl ( ryv );
							rxv = -10.0f * rad + RND * 100.0f * rad;
						  rxx = cosl ( rxv );
							rxy = sinl ( rxv );
							newrender ( );
							clearscreenbufs ( bgcol [ showbackground ] );
							clearscreen ( bgcol [ showbackground ] );
							SunCode ( );
	  	  		break; // A.

						case 'B':
							showbackground++;
              if ( showbackground > 4 )
              	showbackground = 0;
							newrender ( );
							clearscreenbufs ( bgcol [ showbackground ] );
							clearscreen ( bgcol [ showbackground ] );
							SunCode ( );
							printsceneinfo ( );
	  	  		break; // B.

						case 'C':
							clearscreenbufs ( bgcol [ showbackground ] );
	  	  		break; // C.

						case 'F':
							newsetup ( );
              getspot ( );
							clearscreen ( bgcol [ showbackground ] );
							SunCode ( );
							printsetinfo ( );
				      printcoordinfo ( );
	  	  		break; // F.

						case 'I':
							printsetinfo ( );
							printcoordinfo ( );
							printsceneinfo ( );
							ShowPalette ( JLIA );
              lcol = txcol [ showbackground ];
              pixelsmess ( );
	  	  		break; // I.

						case 'L':
              lightness++;
              if ( lightness > 1 )
								lightness = 0;
							newrender ( );
							clearscreenbufs ( bgcol [ showbackground ] );
							clearscreen ( bgcol [ showbackground ] );
							SunCode ( );
							printsceneinfo ( );
	  	  		break; // L.

						case 'N':
					  	ui++;
						  if ( ( ui < 0 ) || ( ui > 9 ) )
					  	 	ui = 0;
							newrender ( );
							clearallbufs ( bgcol [ showbackground ] );
							clearscreen ( bgcol [ showbackground ] );
							SunCode ( );
							printsetinfo ( );
							printcoordinfo ( );
	  	  		break; // N.

						case 'P':
							NewPalette = true;
	  	  		break; // P.

						case 'R':
							newsetup ( );
							clearscreen ( bgcol [ showbackground ] );
							SunCode ( );
							printsetinfo ( );
				      printcoordinfo ( );
	  	  		break; // R.

						case 'S':
					  	seti++;
						  if ( ( seti < 0 ) || ( seti > 7 ) )
					  	 	seti = 0;
							newrender ( );
							clearallbufs ( bgcol [ showbackground ] );
							clearscreen ( bgcol [ showbackground ] );
							SunCode ( );
							printsetinfo ( );
	  	  		break; // S.

						case 'T':
							FILLBOX ( 0, 0, WIDTH, HEIGHT, 0x00FFFFFF );
							SunCode ( );
              lcol = 0x00000080;
              pixelsmess ( );
	  	  		break; // T.

						case 'V':
              showpic ( );
	  	  		break; // V.

						case 'W':
              whitershade++;
              if ( whitershade > 2 )
								whitershade = 0;
							newrender ( );
							clearscreenbufs ( bgcol [ showbackground ] );
							clearscreen ( bgcol [ showbackground ] );
							SunCode ( );
							printsceneinfo ( );
	  	  		break; // W.

 						case 'X':
					   	if ( ( ++nxi ) > 6 )
					     	nxi = 0;
							for ( i = 0; i < 10; i++ )
						  {
						    xbuf [ i ] = 1.0f;
						    ybuf [ i ] = 1.0f;
								zbuf [ i ] = 1.0f;
						  }
							newrender ( );
							clearallbufs ( bgcol [ showbackground ] );
							clearscreen ( bgcol [ showbackground ] );
							SunCode ( );
							printsetinfo ( );
	  	  		break; // X.

 						case 'Z':
            	secretingredient = ( ! secretingredient );
             	secretsquare = RNDBOOL;
              secretextracoord = RNDBOOL;
             	if ( ! secretsquare )
 		            secretextracoord = true;
 	            secretsize = 1.0f + RND * 5.0f;
							newrender ( );
							clearallbufs ( bgcol [ showbackground ] );
							clearscreen ( bgcol [ showbackground ] );
							SunCode ( );
							printsetinfo ( );
	  	  		break; // Z.

					} // wparam (render screen).
				break;

        // Intro screen:
	    	case 1:
					switch ( wParam )
					{
						case VK_ESCAPE:
			    		PostMessage ( hWnd, WM_CLOSE, 0, 0 );
			    	break; // Escape.

						case VK_SPACE:
							programMode = 0;
			    		PostMessage ( hWnd, WM_PAINT, 0, 0 );
		    		break; // Space.

						case 'B':
							showbackground++;
              if ( showbackground > 4 )
              	showbackground = 0;
							newrender ( );
							newset = true;
							printsceneinfo ( );
	  	  		break; // B.

						case 'F':
            	temp = pcz [ ui ];
							newsetup ( );
              getspot ( );
							newset = true;
				      printcoordinfo ( );
							ShowPalette ( JLIA );
	  	  		break; // F.

						case 'I':
							printsetinfo ( );
							printcoordinfo ( );
							printsceneinfo ( );
							ShowPalette ( JLIA );
	  	  		break; // I.

						case 'L':
              lightness++;
              if ( lightness > 1 )
								lightness = 0;
							newrender ( );
							newset = true;
							printsceneinfo ( );
	  	  		break; // L.

						case 'N':
			    		ui++;
      				if ( ( ui < 0 ) || ( ui > 9 ) )
      					ui = 0;
							newrender ( );
							newset = true;
      				printcoordinfo ( );
							ShowPalette ( JLIA );
	  	  		break; // N.

						case 'P':
    					CreatePalette ( );
      				ShowPalette ( ABSZ );
	  	  		break; // P.

						case 'R':
							newsetup ( );
							newset = true;
				      printcoordinfo ( );
							ShowPalette ( JLIA );
	  	  		break; // R.

						case 'S':
					  	seti++;
						  if ( ( seti < 0 ) || ( seti > 7 ) )
					  	 	seti = 0;
							newset = true;
		    		  printsetinfo ( );
	  	  		break; // S.

						case 'W':
              whitershade++;
              if ( whitershade > 2 )
								whitershade = 0;
							newrender ( );
							newset = true;
							printsceneinfo ( );
	  	  		break; // W.

						case 'X':
				    	if ( ( ++nxi ) > 6 )
      					nxi = 0;
      				for ( i = 0; i < 10; i++ )
      				{
    		  			xbuf [ i ] = 1.0f;
		        		ybuf [ i ] = 1.0f;
		        		zbuf [ i ] = 1.0f;
    				  }
							newrender ( );
							newset = true;
		    		  printsetinfo ( );
            break; // X.

						case 'Z':
            	secretingredient = ( ! secretingredient );
             	secretsquare = RNDBOOL;
              secretextracoord = RNDBOOL;
             	if ( ! secretsquare )
 		            secretextracoord = true;
 	            secretsize = 1.0f + RND * 5.0f;
							newrender ( );
							newset = true;
							printsetinfo ( );
            break; // Z.

					} // wparam (info screen).

				break;
	    }
		break; // VM_KEYDOWN.

    case WM_DESTROY:
			renderactive = false;
			PostQuitMessage ( 0 );
		break;

    default:
	  return ( DefWindowProc ( hWnd, message, wParam, lParam ) );

  } // switch message.
  return ( DefWindowProc ( hWnd, message, wParam, lParam ) );
} // WindowProc.
/* --------------------------------------------------------------------- *
			Destroyer:
 * --------------------------------------------------------------------- */
static void finiObjects(void)
{
	if ( lpDD != NULL )
  {
		if ( lpDDSBuffer != NULL )
		{
	    lpDDSBuffer -> Release ( );
	    lpDDSBuffer = NULL;
		}
		lpDD -> Release ( );
		lpDD = NULL;
  }
} /* finiObjects */
/* --------------------------------------------------------------------- *
			Setup: 	randomize,
      				create some constants,
      				create window,
              set screen-mode,
              get pointer to screen.

 * --------------------------------------------------------------------- */
static BOOL doInit ( HINSTANCE hInstance, int nCmdShow )
{
	// Randomize & Create constants:
	randomize ( );
  phi = ( 1.0f + sqrtl ( 5.0f ) ) / 2.0f;
  pii = 2.0f * pi;
	rad = pii / 360.0f;
	RATIO	=	WIDTH / HEIGHT;
	RERAT	=	HEIGHT / WIDTH;

	// setup & register window: (who needs more? =)
  wc.lpfnWndProc = WindowProc;
  wc.hInstance = hInstance;
  wc.lpszClassName = NAME;
  RegisterClass ( &wc );

	// Create the window:
  hwnd = CreateWindow ( NAME, 0, 0, 0, 0, 0, 0, NULL, NULL, hInstance, NULL );
 	if( ! hwnd )
		return ( false );

  // Open the window & hide the mouse-pointer:
	ShowWindow ( hwnd, nCmdShow );
  SetCursor ( NULL );

  // Create the main DirectDraw object:
	ddrval = DirectDrawCreate ( NULL, &lpDD, NULL );
  if ( ddrval == DD_OK )
  {
		// Get exclusive mode:
		ddrval = lpDD -> SetCooperativeLevel ( hwnd, DDSCL_EXCLUSIVE | DDSCL_FULLSCREEN );
		if ( ddrval == DD_OK )
		{
			// Open screen-mode:
    	ddrval = lpDD -> SetDisplayMode ( WIDTH, HEIGHT, 32 );
    	if ( ddrval == DD_OK )
    	{
				// Create a surface:
				ddsd.dwSize = sizeof ( ddsd );
				ddsd.dwFlags = DDSD_CAPS;
				ddsd.ddsCaps.dwCaps = DDSCAPS_PRIMARYSURFACE;
				ddrval = lpDD -> CreateSurface ( &ddsd, &lpDDSBuffer, NULL );
			  lpDDSBuffer -> Lock ( NULL, &ddsd, DDLOCK_SURFACEMEMORYPTR | DDLOCK_WAIT, NULL );
			 	lpBuf = ( long* ) ddsd.lpSurface;
			  lk = ddsd.lPitch >> 2;
			  lpDDSBuffer -> Unlock ( lpBuf );
				// Exit good:
       	return true;
  		}
  	}
  }

	// if something got messed up:
	finiObjects ( );
	DestroyWindow ( hwnd );

	return ( false );
} // doInit.
/* --------------------------------------------------------------------- *
										 	 ___								 		 _
											// \\  	 _   |	 _	 	 	 \\
									   //	 	 		 \\  |	//		 	 //
									  //		  		\\___//		 	 	//
						  			\\____	 		//   \\   	 //
										   	 \\ ___//		  \\___	//
													\\	 \\		  //   //
									 _      //	  \\___//   //
									 \\    //	  	//   \\  //			_
									  \\__//	 	 //  |  \\ \\____//
									   \--/			/    |  	\ \----/

 * --------------------------------------------------------------------- *
			Program setup:
/* --------------------------------------------------------------------- */
void initiateIFS ( void )
{
  // Create a fractal set:
	newsetup ( );

	// Also create a colour-palette:
	CreatePalette ( );

  // Background is visible:
  showbackground = 0;

  // Clear buffers:
	clearallbufs ( bgcol [ showbackground ] );

	return;
}// End of initiateIFS.
/* --------------------------------------------------------------------- *
		Here goes the iteration-loop:
 * --------------------------------------------------------------------- */
void DoMyStuff ( void )
{
	// If someone's tap'n the [Alt] key:
	if ( GetAsyncKeyState ( VK_MENU ) )
  {
		renderactive = false;
		programMode = 1;
 		PostMessage ( hwnd, WM_PAINT, 0, 0 );
  	return;
  }

	// Don't count pixels written for bottom plane:
	// Comment: The counter is updated inside the plot function.
	//          This is why I save the value, (to restore later).
	//          Ok! I could use a flag instaed, but did not =)
	spixelswritten = pixelswritten;
	sshadowswritten = shadowswritten;

	// Show background?
	if ( showbackground == 0 )
  {
	  // ************************* //
  	// * Bottom ******** IFS ! * //
  	// ************************* //

		// Select scale size:
		pixscale = 1.0f;

		// Turn glow on:
  	useglow = true;

    // Don't write to shadow map:
    doshadow = false;

		// Iteration loop:
	  for ( pti = 128; pti >= 0; pti-- )
  	{
  		bi = int ( RND * 4 );

	    btx = ( btx - tx [ bi ] ) * sc [ bi ];
  	  bty = ( bty - ty [ bi ] ) * sc [ bi ];
    	btz = ( btz - tz [ bi ] ) * sc [ bi ];

	    // Attractor-glow:
			if ( useglow )
 	    {
	  	  t = sqrtl ( btx*btx + bty*bty + btz*btz );
  	  	if ( t > blargel )
	  	  {
  	  		blargel = t;
	    	}	// if blargel.
		    t = pow ( ( 1.0f - t / blargel ), 16.0f );
				bglow = ( bglow + t ) / 2.0f;
      } // Attractor-glow.

	    btx += tx [ bi ];
  	  bty += ty [ bi ];
    	btz += tz [ bi ];

			// Color:
  	  bcr = ( ( bcr + tcr [ bi ] ) >> 1 ) & 0xFF;
    	bcg = ( ( bcg + tcg [ bi ] ) >> 1 ) & 0xFF;
	    bcb = ( ( bcb + tcb [ bi ] ) >> 1 ) & 0xFF;

//			if ( bi & 0x1 )
//				pali += ( ( PALSIZE - pali ) >> 4 );
//			else
//				pali >>= 4;
//
//			tcolor = lpCols [ pali ];
//			tRed = 0xFF ^ ( ( tcolor >> 17 ) & 0x7F );
//			tGreen = 0xFF ^ ( ( tcolor >> 9 ) & 0x7F );
//			tBlue = 0xFF ^ ( ( tcolor >> 1 ) & 0x7F );
//			bcr = ( ( bcr + tRed ) >> 1 ) & 0xFF;
//			bcg = ( ( bcg + tGreen ) >> 1 ) & 0xFF;
//			bcb = ( ( bcb + tBlue ) >> 1 ) & 0xFF;
			// color.

 	   // Scale & translate to scene:
  	  xt = btx * pixscale + CPOSX;
    	yt = bty * pixscale + CPOSY;
	    zt = btz * pixscale + CPOSZ;

			// it's light:
  	  glow = bglow;
	    IFSlight ( );

	    // Scale & translate to scene:
  	  xt = btx * pixscale + CPOSX;
    	yt = bty * pixscale + CPOSY;
	    zt = btz * pixscale + CPOSZ;

			// Select colour for pixel:
  	  crt = bcr;
    	cgt = bcg;
	    cbt = bcb;

	    // Plot pixel to scene:
	    IFSplot ( );

	  } // End of the iteration loop (the bottom-plane).
	} // if showbackground.

  // ************************//
  // * Fractal ******* IFS ! //
  // ************************//
//			ui = int ( RND * 9 );

	// select repetisions:
	maxrepti = int ( RND * RND * 128 );
  // reset repeat counter:
	repti = -1;

  // Reset coordinate:
	dtx = xbuf [ ui ];
	dty = ybuf [ ui ];
	dtz = zbuf [ ui ];

	// Select scale size:
	pixscale = 0.25f;

	// Turn glow off:
  useglow = false;

  // Write to shadow map:
  doshadow = true;

	indxn = 0;
	if ( RNDBOOL )
  	indxn = -1;

	useswap = RNDBOOL;

	// Restore iteration counter:
	pixelswritten = spixelswritten;
  shadowswritten = sshadowswritten;

  // Iteration loop:
  for ( pti = 512; pti >= 0; pti-- )
  {
    // Update ireration counter:
		itersdone++;

		dtx -= pcx [ ui ];
    dty -= pcy [ ui ];
    dtz -= pcz [ ui ];

    // Secretingredient 1: (1 + 2 = both methods)
		if ( secretingredient )
    {
			pixscale = 0.15f;
    	if ( secretextracoord & int ( RND * 2 ) )
      {
				if ( ! int ( RND * 3 ) )
  		  	dtx += ( secretsize * RNDSGN );
    		else if ( int ( RND * 2 ) )
	    		dty += ( secretsize * RNDSGN );
  		  else
		   		dtz += ( secretsize * RNDSGN );
      }
    }

   // Secretingredient 2: (1 + 2 = both methods)
		if ( secretsquare && secretingredient )
		{
			pixscale = 0.25f;
			t = dty;
      dty = - dtz;
      dtz = - t;
    } // if  secretingredient & secretsquare.

    switch ( seti )
    {
    	case 1:
      	SETB;
      break;
      case 2:
      	SETC;
      break;
      case 3:
      	SETD;
      break;
      case 4:
      	SETE;
      break;
      case 5:
      	SETD3;
      break;
      case 6:
      	SET2D;
      break;
      case 7:
      	SET2D3;
      break;
      default:
      	SETA;
      break;
    } // set selector.

    // x selector:
    if ( ( --repti ) <= 0 )
    {
    	repti = maxrepti;
      sduoi = int ( RND * 2 );
      smulti = int ( RND * 8 );
      probability = RND;
    }
    duoi = sduoi;
    multi = smulti;
    palupflag = false;

    if ( indxn < 0 )
    	duoi = ( RND < probability );
    else
    {
			if ( indxn == 0)
      {
        indxs++;
				indxuse = indxs;
      	indxn = 24;
      }
			if ( useswap )
      {
	      swapflag = ( ! swapflag );
				if( swapflag )
    			duoi = ( ! ( indxuse & 0x1 ) );
      	else
    			duoi = ( indxuse & 0x1 );
      }
      else
   			duoi = ( indxuse & 0x1 );

      indxuse >>= 1;
      --indxn;
    }

    switch ( nxi )
    {
    	case 1:
      	MOD3X;
    	case 2:
      	MOD4X;
    	case 3:
      	MOD6X;
      break;
      case 4:
      	MOD6XX;
      break;
      case 5:
      	MOD8X;
      break;
      case 6:
      	MOD2X6X;
      break;
      default:
      	MOD2X;
      break;
    } // x selector.

		if ( palupflag )
    {
    	pali += ( PALSIZE - pali ) >> 2;
      pali2 =  pali - ( pali >> 2 );
    }
    else
    {
    	pali = pali - ( pali >> 2 );
      pali2 += ( PALSIZE - pali ) >> 2;
    }

    switch ( pmodi )
    {
    	case 1:
      	COLMOD;
      break;
      default:
      	COLPAL;
      break;
    } // color selector.

    // Scale & translate to scene:
    xt = dtx * pixscale + CPOSX;
    yt = ( -dty ) * pixscale + CPOSY;
    zt = dtz * pixscale + CPOSZ;

		// Save position:
		tmpx = xt;
    tmpy = yt;
    tmpz = zt;

		// Get luminousity & plot in the light's Z-table:
    IFSlight ( );

    // The following section adds a extra shadow pixel
    // at the backside (from the light), of the main pixel:

    // ********************
    // Create shadow pixel:
    // ********************

    // Make pixel a shadowed pixel:
    // Save brightness:
		tbright = bright;
    toverexpose = overexpose;
    tluma = luma;
    // always use half shadow level of brightness:
  	bright = blight;
		overexpose = 0;
		luma = 1.0f;

   	// Rotate to angle of light:
	  rotatelight ( );

		// Move to one 'point' behind the main pixel:
    tmpz += ( 1.0f / ( ims * imszoom ) );

		// Rotate back to original angle:
		unrotatelight ( );

		// ****************************
		// Write shadow pixel to scene:
		// ****************************

		// Coordinate to plot:
  	xt = tmpx;
  	yt = tmpy;
  	zt = tmpz;

		// Select colour for pixel:
  	crt = dcr;
  	cgt = dcg;
  	cbt = dcb;

    // Plot pixel to scene:
  	IFSplot ( );

		// *********************
		// Write pixel to scene:
		// *********************

    // Restore brightness:
		bright = tbright;
    overexpose = toverexpose;
    luma = tluma;

    // Scale & translate to scene:
  	xt = dtx * pixscale + CPOSX;
  	yt = ( -dty ) * pixscale + CPOSY;
    zt = dtz * pixscale + CPOSZ;

		// Select colour for pixel:
    crt = dcr;
    cgt = dcg;
    cbt = dcb;

    // Plot pixel to scene:
    IFSplot ( );

		// *** NOW! A NEW IDÈA: WRITE THE "SECOND ROOT" ****
    // Only used for some of the x-modes:
    if ( ( nxi == 0 ) ||\
    		 ( nxi == 1 ) ||\
    		 ( nxi == 2 ) )

//         ||\
//         ( nxi == 5 ) )
    {
			// Store colour & index:
			tcolr = dcr;
			tcolg = dcg;
			tcolb = dcb;
  	  tmp = pali;
			pali = pali2;
	    pali2 = tmp;
      palupflag = ( ! palupflag );
    	switch ( pmodi )
	    {
  	  	case 1:
    	  	COLMOD;
      	break;
	      default:
  	    	COLPAL;
    	  break;
	    } // color selector.

	    // Scale & translate to scene:
  	  xt = ( -dtx ) * pixscale + CPOSX;
    	yt = dty * pixscale + CPOSY;
	    zt = ( -dtz ) * pixscale + CPOSZ;

			// Save position:
			tmpx = xt;
  	  tmpy = yt;
    	tmpz = zt;

			// Get luminousity & plot in the light's Z-table:
  	  IFSlight ( );

	    // The following section adds a extra shadow pixel
  	  // at the backside (from the light), of the main pixel:

    	// ********************
	    // Create shadow pixel:
  	  // ********************

    	// Make pixel a shadowed pixel:
	    // Save brightness:
			tbright = bright;
	    toverexpose = overexpose;
  	  tluma = luma;
    	// always use half shadow level of brightness:
	  	bright = blight;
			overexpose = 0;
			luma = 1.0f;

	   	// Rotate to angle of light:
		  rotatelight ( );

			// Move to one 'point' behind the main pixel:
  	  tmpz += ( 1.0f / ( ims * imszoom ) );

			// Rotate back to original angle:
			unrotatelight ( );

			// ****************************
			// Write shadow pixel to scene:
			// ****************************

			// Coordinate to plot:
  		xt = tmpx;
  		yt = tmpy;
	  	zt = tmpz;

			// Select colour for pixel:
  		crt = dcr;
	  	cgt = dcg;
  		cbt = dcb;

	    // Plot pixel to scene:
  		IFSplot ( );

			// *********************
			// Write pixel to scene:
			// *********************

	    // Restore brightness:
			bright = tbright;
    	overexpose = toverexpose;
	    luma = tluma;

    	// Scale & translate to scene:
	  	xt = ( -dtx ) * pixscale + CPOSX;
  		yt = dty * pixscale + CPOSY;
    	zt = ( -dtz ) * pixscale + CPOSZ;

			// Select colour for pixel:
  	  crt = dcr;
    	cgt = dcg;
	    cbt = dcb;

  	  // Plot pixel to scene:
    	IFSplot ( );

			// Restore colour:
			dcr = tcolr;
			dcg = tcolg;
			dcb = tcolb;
			pali = pali2;
		} // End of second root extra pixel.

	} // End of the iteration loop (the julia-set).

  // Save current coordinate:
  xbuf [ ui ] = dtx;
  ybuf [ ui ] = dty;
  zbuf [ ui ] = dtz;

  // if flag true? - create a new palette:
  if ( NewPalette )
  {
  	CreatePalette ( );
    ShowPalette ( ABSZ );
  } // if NewPal.

	return;
}// End of DoMyStuff (iteration-loop).
/* --------------------------------------------------------------------- *
			Make it light:

	      Get luminousity, plot in light scene Z-table.

 * --------------------------------------------------------------------- */
void IFSlight ( void )
{
	// Rotate to light position:
  rotatelight ( );

	// Clip z:
	if ( ( zt > -1.0f ) && ( zt < 1.0f ) )
	{
		// Get distance to light:
    size = ( 3.0f + zt ) / 2.0f;
    t = ( 2.0f - size );

    // Calculate & calibrate luminousity:
    t = ( 1.0f + t ) / 2.0f;

    if ( t < minbright )
    	minbright = t;
    t = t - minbright;
    if ( t > maxbright )
    	maxbright = t;
    t = t / maxbright;

		t = t * 2.0f;
	  luma = t - 1.0f;
  	if ( luma < 0.0f )
	  	luma = 0.0f;
	  luma = 1.0f + powl ( luma, 8.0f );
		if ( t > 1.0f )
  		t = 1.0f;
	  overexpose = int ( 255.0 * luma ) - 0xFF;
  	if ( overexpose < 0 )
  		overexpose = 0;

		if ( useglow )
		  bright = int ( 48 * glow + 208 * t ) & 0xFF;
    else
			bright = int ( 255 * t ) & 0xFF;

		blight = bright >> 1;

	  nZ = int ( ( 2.0f - size ) * ( ZDEPTH >> 1 ) ) & 0x7FFF;
  	zt = ( lims * imszoom ) / size;

	  nY = LMIDY + int ( yt * zt );
  	nX = LMIDX + int ( xt * zt );
		// Clip y:
	  if ( ( nY >= 0 ) && ( nY < LHEIGHT ) )
  	{
			// Clip x:
  		if ( ( nX >= 0 ) && ( nX < LWIDTH ) )
	    {
    	  if ( light [ nY ] [ nX ] > ( nZ ) )
      	{
	      	// Create shadow.
  	      bright = blight;
    	    overexpose = 0;
      	  luma = 1.0f;
	      }
  	    else if ( doshadow )
    	  {
					// Update counter for pixels written to shadow map:
	    	  if ( light [ nY ] [ nX ] < nZ )
          	shadowswritten++;
      		light [ nY ] [ nX ] = nZ;
	      } // Shadow or not.
  	  } // clip - X.
	  }	// clip - Y.
	} // clip - Z.

	return;
} // IFSlight.
/* --------------------------------------------------------------------- *
			Make it show:

      	Rotate to scene ¹, plot to Z-table and pixel-buffer,
      	anti-anilize and plot to screen from pixel-buffer.

				¹ No translation, just views towards the center here =)

 * --------------------------------------------------------------------- */
void IFSplot ( void )
{
	// Rotate to angle of view:
  rotateview ( );

	// Clip z:
  if ( ( zt > -1.0f ) && ( zt < 1.0f ) )
  {
  	size = ( 3.0f + zt ) / 2.0f;
    nZ = int ( ( 2.0f - size ) * ( ZDEPTH >> 1 ) ) & 0x7FFF;
    zt = ( ims * imszoom ) / size;

    nY = BMIDY + int ( yt * zt );
    nX = BMIDX + int ( xt * zt );
		// Clip y:
    if ( ( nY >= 0 ) && ( nY < BHEIGHT ) )
    {
			// Clip x:
  	  if ( ( nX >= 0 ) && ( nX < BWIDTH ) )
	    {
      	// Plot if Point closer to viewer than
        // the previous at the position:
				if ( bpict [ nY ] [ nX ] < nZ )
        {
					// Write new depth to z-buffer:
        	bpict [ nY ] [ nX ] = nZ;

          // Update pixel counter:
          pixelswritten++;

					// Brighter than average?
          if ( overexpose )
          {
	          crt = int ( ( crt + overexpose ) / luma );
  	        cgt = int ( ( cgt + overexpose ) / luma );
						cbt = int ( ( cbt + overexpose ) / luma );
          } // Overexpose.

					// Whiter shade of pale?:
					if ( whitershade )
          {
						// Cold in varm:
						if ( whitershade == 1 )
            {
	          	crt = ( ( ( crt * 3 ) + bright ) >> 2 ) & 0xFF;
  	        	cgt = ( ( ( cgt << 1 ) + bright ) / 3 ) & 0xFF;
    	      	cbt = ( ( cbt  + bright ) >> 1 ) & 0xFF;
            }
						// Varm in cold:
            else
            {
    	      	crt = ( ( crt  + bright ) >> 1 ) & 0xFF;
  	        	cgt = ( ( ( cgt << 1 ) + bright ) / 3 ) & 0xFF;
	          	cbt = ( ( ( cbt * 3 ) + bright ) >> 2 ) & 0xFF;
            }
          } // Whiter shade of pale.

          crt = ( ( crt * bright ) >> 8 ) & 0xFF;
          cgt = ( ( cgt * bright ) >> 8 ) & 0xFF;
          cbt = ( ( cbt * bright ) >> 8 ) & 0xFF;
          tcolor = ( ( crt << 16 ) + ( cgt << 8 ) + cbt ) & 0xFFFFFF;
          pict [ nY ] [ nX ] = tcolor;

					// ******************************
          // Anti anlize from pixel-buffer:
					// ******************************
          // 2x2 grid:
          nY = nY & 0xFFFE;
          nX = nX & 0xFFFE;

          // Reset colours:
					ncols = 4;
          tRed = 0x00;
 	        tBlue = 0x00;
   	      tGreen = 0x00;

					// 2x2 pixels to 1 pixel:
          for ( yi = 0; yi < 2; yi++ )
          {
          	nYt = nY + yi;
            if ( ( nYt >= 0 ) && ( nYt < BHEIGHT ) )
            {
            	for ( xi = 0; xi < 2; xi++ )
              {
              	nXt = nX + xi;
                if ( ( nXt >= 0 ) && ( nXt < BWIDTH ) )
                {
                	tcolor = pict [ nYt ] [ nXt ];
                  tRed += ( tcolor >> 16 ) & 0xFF;
                  tGreen += ( tcolor >> 8 ) & 0xFF;
                  tBlue += tcolor & 0xFF;
                } // Clip x.
              } // for xi.
            } // Clip y.
          } // for yi.
      	  tRed = ( tRed / ncols ) & 0xFF;
 	        tGreen = ( tGreen / ncols ) & 0xFF;
   	      tBlue = ( tBlue / ncols ) & 0xFF;
          // End anti anilize.

					// Convert 8-bit red, green & blue to 32-bit xRGB:
          tcolor = ( ( tRed << 16 ) + ( tGreen << 8 ) + tBlue ) & 0x00FFFFFF;

					// ***********************
          // Write to screen buffer:
					// ***********************
          // 2x2 grid to 1x1 dito:
          nY = nY >> 1;
          nX = nX >> 1;
          lpBuf [ nX + nY * lk ] = tcolor;
				} // Z-plot view.
			} // clip - X.
		} // clip - Y.
	} // clip - Z.

	return;
} // IFSplot.
/* --------------------------------------------------------------------- *
			Rotate to view position:
 * --------------------------------------------------------------------- */
void rotateview ( void )
{
  t = ryx * zt - ryy * xt;
  xt = ryx * xt + ryy * zt;
  zt = t;

  t = rxx * yt - rxy * zt;
  zt = rxx * zt + rxy * yt;
  yt = t;

	return;
} // rotateview.
/* --------------------------------------------------------------------- *
			Rotate back from view position:
 * --------------------------------------------------------------------- */
void unrotateview ( void )
{
  t = ryx * zt - ( -ryy ) * xt;
  xt = ryx * xt + ( -ryy ) * zt;
  zt = t;

  t = rxx * yt - ( -rxy ) * zt;
  zt = rxx * zt + ( -rxy ) * yt;
  yt = t;

	return;
} // unrotatelight.
/* --------------------------------------------------------------------- *
			Rotate to light position:
 * --------------------------------------------------------------------- */
void rotatelight ( void )
{
	t = lryx * zt - lryy * xt;
	xt = lryx * xt + lryy * zt;
	zt = t;

	t = lrxx * yt - lrxy * zt;
	zt = lrxx * zt + lrxy * yt;
	yt = t;

	return;
} // rotatelight.
/* --------------------------------------------------------------------- *
			Rotate back from light position:
 * --------------------------------------------------------------------- */
void unrotatelight ( void )
{
	t = lrxx * yt - ( -lrxy ) * zt;
	zt = lrxx * zt + ( -lrxy ) * yt;
	yt = t;

	t = lryx * zt - ( -lryy ) * xt;
	xt = lryx * xt + ( -lryy ) * zt;
	zt = t;

	return;
} // unrotatelight.
/* --------------------------------------------------------------------- *
			Initiate a new render:
 * --------------------------------------------------------------------- */
void newrender ( void )
{
	// Reset parameters:
  btx = tx [ 0 ];
  bty = ty [ 0 ];
  btz = tz [ 0 ];

	bcr = tcr [ 0 ];
  bcg = tcg [ 0 ];
	bcb = tcb [ 0 ];

  dtx = 1.0f;
  dty = 1.0f;
  dtz = 1.0f;

	dcr = tcr [ 4 ];
  dcg = tcg [ 4 ];
	dcb = tcb [ 4 ];

	// Palette index:
	pali = 0;

	// Min & max brightness:
  switch ( lightness )
  {
  	case 1:
			minbright = ( 1.0f / 3.0f );
    	maxbright = ( 1.0f / 2.0f );
    break;
    default:
    	minbright = 1.0f;
      maxbright = 0.0001f;
    break;
  } // color selector.

	return;
} // newrender.
/* --------------------------------------------------------------------- *
		Initiate a new setup:
 * --------------------------------------------------------------------- */
void newsetup ( void )
{
	ui = 9;

	//////////////////////////////
	// *** FRACTAL SETUPS ! *** //
	//////////////////////////////
	// Bottom-plane, (2D IFS - space fill square):
  x = 0.41f;
  y = 0.45f;

  // Translation cordinates:
  if ( seti > 5 )
  {
		tx [ 0 ] 	= -x;
		ty [ 0 ] 	=  x;
		tz [ 0 ] 	=  y;

		tx [ 1 ] 	=  x;
		ty [ 1 ] 	=  x;
		tz [ 1 ] 	=  y;

		tx [ 2 ] 	=  x;
		ty [ 2 ] 	= -x;
		tz [ 2 ] 	=  y;

		tx [ 3 ] 	= -x;
		ty [ 3 ] 	= -x;
		tz [ 3 ] 	=  y;
  }
  else
  {
		tx [ 0 ] 	= -x;
		ty [ 0 ] 	=  y;
		tz [ 0 ] 	=  x;

		tx [ 1 ] 	=  x;
		ty [ 1 ] 	=  y;
		tz [ 1 ] 	=  x;

		tx [ 2 ] 	=  x;
		ty [ 2 ] 	=  y;
		tz [ 2 ] 	=  -x;

		tx [ 3 ] 	= -x;
		ty [ 3 ] 	=  y;
		tz [ 3 ] 	= -x;
  }

	// Colours:
	tcr [ 0 ] = 0x90;
  tcg [ 0 ] = 0x90;
	tcb [ 0 ] = 0x90;

	tcr [ 1 ] = 0x70;
  tcg [ 1 ] = 0x70;
	tcb [ 1 ] = 0x70;

	tcr [ 2 ] = 0x90;
  tcg [ 2 ] = 0x90;
	tcb [ 2 ] = 0x90;

	tcr [ 3 ] = 0x70;
  tcg [ 3 ] = 0x70;
	tcb [ 3 ] = 0x70;

	// Scale ratios:
	for ( i = 0; i < 4; i++ )
		sc [ i ] = ( 1.0f / 2.0f );

  // End bottom.

  // Fractal set:
	x = 0.5f;
  y = 0.3f;

	// Presets coordiantes:

	pcx [ 0 ] = 0.0f;
  pcy [ 0 ] = 0.0f;
  pcz [ 0 ] = 0.0f;

	pcx [ 1 ] = x;
  pcy [ 1 ] = -y;
  pcz [ 1 ] = 0.0f;

	pcx [ 2 ] = -1.414289f;
  pcy [ 2 ] = 0.0f;
  pcz [ 2 ] = 0.0f;

	pcx [ 3 ] = 0.285f;
  pcy [ 3 ] = 0.013f;
  pcz [ 3 ] = 0.0f;

	pcx [ 4 ] = 0.4f;
  pcy [ 4 ] = 0.3f;
  pcz [ 4 ] = 0.5f;

	pcx [ 4 ] = sqrtl ( 2.0f ) / 4.0f;
  pcy [ 4 ] = 0.0f;
  pcz [ 4 ] = 0.0f;

	pcx [ 5 ] = 0.387860f;
  pcy [ 5 ] = 0.154406f;
  pcz [ 5 ] = 1.0f;

	pcx [ 6 ] = -.6875f;
  pcy [ 6 ] = -.0625f;
  pcz [ 6 ] = -.24849984f;

	pcx [ 7 ] = -0.717612232f;
  pcy [ 7 ] =  0.217535936f;
  pcz [ 7 ] =  y;

	pcx [ 8 ] = -0.25f;
  pcy [ 8 ] = 0.5f;
  pcz [ 8 ] = 0.75;

	pcx [ 9 ] = RND * RND * RND * 5.0f * SGN ( 0.5f - RND );
	pcy [ 9 ] = RND * RND * 3.0f * SGN ( 0.5f - RND );
	pcz [ 9 ] = RND * RND * 5.0f * SGN ( 0.5f - RND );

	// reset position buffers:
	for ( i = 0; i < 10; i++ )
  {
    xbuf [ i ] = 1.0f;
    ybuf [ i ] = 1.0f;
    zbuf [ i ] = 1.0f;
  } // for i.

	// Colours:
  // (Note the index numbers: The colour-table is
	// shared with the bottom-plane IFS (above)):

  tcr [ 4 ] = 0xFF;
	tcg [ 4 ] = 0x00;
  tcb [ 4 ] = 0x50;

  tcr [ 5 ] = 0xFF;
	tcg [ 5 ] = 0x80;
  tcb [ 5 ] = 0x00;

  tcr [ 6 ] = 0xFF;
	tcg [ 6 ] = 0xFF;
  tcb [ 6 ] = 0x00;

  tcr [ 7 ] = 0x80;
	tcg [ 7 ] = 0xC0;
  tcb [ 7 ] = 0x00;

  tcr [ 8 ] = 0x00;
	tcg [ 8 ] = 0xC0;
  tcb [ 8 ] = 0x40;

  tcr [ 9 ] = 0x00;
	tcg [ 9 ] = 0x80;
  tcb [ 9 ] = 0xC0;

  tcr [ 10 ] = 0x00;
	tcg [ 10 ] = 0x00;
  tcb [ 10 ] = 0xFF;

  tcr [ 11 ] = 0x80;
	tcg [ 11 ] = 0x00;
  tcb [ 11 ] = 0xFF;
	// End Fractal.

	// IFS rotators:
	drzv = 45.0f * rad;
	dryv = 120.0f * rad;
  drzx = cosl ( drzv );
  drzy = sinl ( drzv );
  dryx = cosl ( dryv );
  dryy = sinl ( dryv );

/// ******* Light & Camera ******* ///

	// Light-angle:
	lryv = 60.0f * rad;
	lrxv = 60.0f * rad;
 	LitAng ( );

	// Camera-angle:
	ryv = -180.0f * rad + RND * 360.0f * rad;
	rxv = -5.0f * rad + RND * 95.0f * rad;
  if ( seti > 5 )
  {
		ryv = 0.0f * rad;
		rxv = 0.0f * rad;
  }
  CamAng ( );
	// Camera-angle.

	newrender ( );
	clearallbufs ( bgcol [ showbackground ] );

	// Angle for 3X mod:
  r3Xv = 120.0f * rad;
	r3Xx = cosl ( r3Xv );
	r3Xy = sinl ( r3Xv );

	return;
} // newsetup.
/* --------------------------------------------------------------------- *
		Get Julia coordinate:
 * --------------------------------------------------------------------- */
void getspot ( void )
{
	if ( seti > 5 )
  {
		i = 0;
		while ( ( i < 1 ) || ( i > 80 ) )
	  {
			a = RND * 2.0 * SGN ( 0.5f - RND );
  		b = RND * 2.0 * SGN ( 0.5f - RND );
	    x = 0.0f;
  	  y = 0.0f;
   		i = 128;
	 	  while ( --i )
  	  {
				t = x*x - y*y + a;
      	y = 2.0f * x * y + b;
	      x = t;
  	    if ( ( x*x + y*y ) > 4.0f )
    	  	break;
	    }
			// If the function get stuck, (if a lot of iteraions is used)
  	  if ( GetAsyncKeyState ( 'G' ) )
    		break;
	  }
		pcx [ ui ] = a;
		pcy [ ui ] = b;
		pcz [ ui ] = temp;
  }
  else
  {
		i = 0;
		while ( ( i < 1 ) || ( i > 80 ) )
	  {
			a = RND * 2.0 * SGN ( 0.5f - RND );
  		b = RND * 2.0 * SGN ( 0.5f - RND );
  		c = RND * 2.0 * SGN ( 0.5f - RND );
	    x = 0.0f;
  	  y = 0.0f;
  	  z = 0.0f;
   		i = 128;
	    while ( --i )
  	  {
  	    length = sqrtl ( y*y + z*z );
    	  if ( length > 0.0f )
      	{
	      	tmpy = y / length;
  	      tmpz = z / length;
    	  }
	      else
  	    {
    	  	tmpy = 1.0f;
      	  tmpz = 0.0f;
	      }
 	    	y = length;
   	    t = x * x - y * y;
     	  y = 2.0f * x * y;
       	x = t + a;
        z = tmpz * y + c;
 	      y = tmpy * y + b;
  	    if ( ( x*x + y*y + z*z ) > 4.0f )
 	  	  	break;
	    }
			// If the function get stuck, (if a lot of iteraions is used)
  	  if ( GetAsyncKeyState ( 'G' ) )
    		break;
	  }
		pcx [ ui ] = a;
		pcy [ ui ] = b;
		pcz [ ui ] = c;
  }
} // Get Julia coordinate.
/* --------------------------------------------------------------------- *
		Create light angle:
 * --------------------------------------------------------------------- */
void LitAng ( void )
{
	lryx = cosl ( lryv );
	lryy = sinl ( lryv );
	lrxx = cosl ( lrxv );
	lrxy = sinl ( lrxv );
} // LitAng.
/* --------------------------------------------------------------------- *
		Create camera angle:
 * --------------------------------------------------------------------- */
void CamAng ( void )
{
	ryx = cosl ( ryv );
	ryy = sinl ( ryv );
	rxx = cosl ( rxv );
	rxy = sinl ( rxv );
} // CamAng.
/* --------------------------------------------------------------------- *
		Print my line:
 * --------------------------------------------------------------------- */
void SunCode ( void )
{
  TEXTBOX ( 10, 10, 67, 26, 0x00A0A0A0, 0x00FFFFFF );
	textline ( 10, 10, "SunCode", BIGFONT, 0x00400000 );
	return;
} // SunCode.
/* --------------------------------------------------------------------- *
			"Manual" =)
 * --------------------------------------------------------------------- */
void manual ( void )
{
	clearscreen ( 0x00FFF0E0 );

  // Head texts:
  printsetinfo ( );
  printcoordinfo ( );
  printsceneinfo ( );
  lcol = 0x00000080;
  textline ( 10, 10, "SunCode's 3D Reversed Julia IFS Demo, (first version seventeenth edition of April 2005)", BIGFONT, lcol );

  // Keys:
  TEXTBOX ( 30, 38, 556, 258, 0x00A0A0A0, 0x00D8F8E8 );
  lcol = 0x00006000;
  tmp = 48;
  yi = 56;
  xi = 15;
  textline ( tmp - 16, 38, "Keys to use in this demo:", BIGFONT, lcol );
  textline ( tmp, yi + xi * 0,  "[Esc] = Exit program! | [SPACE] = Toggle this screen and render mode screen, ([Alt] = exit from render)", MEDIUMFONT, lcol );
  textline ( tmp, yi + xi * 1,  "[R] = Randomize setup, (angle of view and c-coordinate), [F] = Finds a coordinate, (if 2D z-part is unchanged).", MEDIUMFONT, lcol );
  textline ( tmp, yi + xi * 2,  "[A] = Randomize angle of view | [Arrow keys] = rotates angle.", MEDIUMFONT, lcol );
  textline ( tmp, yi + xi * 3,  "[P] = Randomize palette, (press until you feel pleased =)", MEDIUMFONT, 0x00008000 );
  textline ( tmp, yi + xi * 4,  "[C] = Clear buffers, (useful after, for example a palette selection to clear out old pixels)", MEDIUMFONT, lcol );
  textline ( tmp, yi + xi * 5,  "[V] = Clear view, (useful after a selection, also removes my SunCode 'tag' from the screen =)", MEDIUMFONT, lcol );
  textline ( tmp, yi + xi * 6,  "[S] = Toggle set, (a, b, c, d, e, also Set d3, the normal 2D and 2D3)", MEDIUMFONT, lcol );
  textline ( tmp, yi + xi * 7,  "[X] = Toggle X-mod, (2X, 3X, 4X, 6X, 6XX, 8X, 2X6X) | [Z] = ya dunno what ya get =)", MEDIUMFONT, lcol );
  textline ( tmp, yi + xi * 8,  "[N] = Next coordinate, (from 10 diffrent preset coordinates, 0 = random)", MEDIUMFONT, lcol );
  textline ( tmp, yi + xi * 9,  "[I] = View set information. | [T] = Test render, a white sheet, are pixels still written?, this will show that.", MEDIUMFONT, lcol );
  textline ( tmp, yi + xi * 10, "[B] = Turn background on, off blue, off black, off grey, or off white.", MEDIUMFONT, lcol );
  textline ( tmp, yi + xi * 11, "[Page Down] / [Page Up] = Zoom in / out | [Home] = Reset zoom.", MEDIUMFONT, lcol );
  textline ( tmp, yi + xi * 12, "[W] = Turn the whiter shade of pale to normal, flourescent or filament | [L] toggle lights = dark or light", MEDIUMFONT, lcol );

  // Palette:
  TEXTBOX ( 30, 270, 312, 568, 0x00A0A0A0, 0x00E0E0E0 );
	ShowPalette ( ABSZ );
  lcol = 0x00303030;
  textline ( 32, 270, "Palette:", BIGFONT, lcol );
  textline ( 132, 556, "Press [P] to randomize", SMALLFONT, lcol );

  // Notes:
  TEXTBOX ( 322, 270, 784, 568, 0x00A0A0A0, 0x00D8E8F8 );
  tmp = 340;
  lcol = 0x00600000;
  textline ( tmp - 16, 270, "Notes:", BIGFONT, lcol );

  textline ( tmp, 290, "To save the current image you can press [Print Screen] and then [Esc] (to exit), and then the", MEDIUMFONT, lcol );
  textline ( tmp, 302, "image is, (hopefully) on your clipboard, ready to get pasted into any graphics tool.", MEDIUMFONT, lcol );

  textline ( tmp, 322, "The palette is only the guide-line for the colouring on screen, the function is: new col=(old col", MEDIUMFONT, lcol );
  textline ( tmp, 334, "+ selected col)/2). Also: the palette viewer does only show a few of the 8192 colours it has got.", MEDIUMFONT, lcol );

  textline ( tmp, 354, "Some of the X-modes uses 6 or 8 fixed colours that can not get changed.", MEDIUMFONT, lcol );

  textline ( tmp, 374, "Not all sets are wery good in all possible combinations. When I constructed the sets I first came", MEDIUMFONT, lcol );
  textline ( tmp, 386, "up with a then b, c, and then at last (after 3 or 4 days) the final solution 'Set d', (did put all in", MEDIUMFONT, lcol );
  textline ( tmp, 398, "here for 'historical reasons', now also added 'Set e'). Most true to 2D Julia is 'Set d 2X', good", MEDIUMFONT, lcol );
  textline ( tmp, 410, "ones are, for example the previous, 'Set d 2X6X', 'Set a 8X' and 'Set b 8X'.", MEDIUMFONT, lcol );

  textline ( tmp, 430, "Keep in mind these functions are using reversed technology. Some of the points are wery hard to", MEDIUMFONT, lcol );
  textline ( tmp, 442, "reach, (fix a bit now) by the method and the image might never get compleated because of this.", MEDIUMFONT, lcol );

	textline ( tmp, 462, "A good idéa is to render the image once to fill out the shadow-map and then press [C] to clear", MEDIUMFONT, lcol );
  textline ( tmp, 474, "out the pixel and z-buffers and [V] to clear the view. And then render the image once again", MEDIUMFONT, lcol );
  textline ( tmp, 486, "using the previously compleated shadow-map. This will make the shadows better.", MEDIUMFONT, lcol );

  textline ( tmp, 506, "The keys [A], [B] and [P] does not destoy the shadow-map. Once the map is filled one can", MEDIUMFONT, lcol );
  textline ( tmp, 518, "press any of these keys to alter the scene and then render a new image using the same", MEDIUMFONT, lcol );
  textline ( tmp, 530, "shadows for faster compleation, (the position of the light is static and will never change).", MEDIUMFONT, lcol );

  textline ( tmp, 550, "This program is part of the public domain, (PD), distribute and make copys freely.", MEDIUMFONT, lcol );

	spacemess ( );

	return;
} // manual.
/* --------------------------------------------------------------------- *
		Press space to ...:
 * --------------------------------------------------------------------- */
void spacemess ( void )
{
  if ( itersdone == 0 )
  {
  	sprintf ( stringbuf, "Press space to start render! %i iterations done, %i image pixels, %i shadow pixels.", itersdone, pixelswritten, shadowswritten );
    textline ( 64, 574, stringbuf, BIGFONT, lcol );
  }
  else
  {
  	sprintf ( stringbuf, "Press space to continue render! %i iterations done, %i image pixels, %i shadow pixels.", itersdone, pixelswritten, shadowswritten );
    textline ( 64, 574, stringbuf, BIGFONT, lcol );
  }
} // spacemess.
/* --------------------------------------------------------------------- *
		Print number of pixels written:
 * --------------------------------------------------------------------- */
void pixelsmess ( void )
{
 	sprintf ( stringbuf, "%i iterations done, %i image pixels, %i shadow pixels.  (press [V] to clear view).", itersdone, pixelswritten, shadowswritten  );
	textline ( 192, 2, stringbuf, MEDIUMFONT, lcol );
} // pixelsmess.
/* --------------------------------------------------------------------- *
		Print info about fractal set:
 * --------------------------------------------------------------------- */
void printsetinfo ( void )
{
  TEXTBOX ( 700, 10, 784, 26, 0x00A0A0A0, 0x00FFFFFF );
	textline ( 702, 10, settexts [ seti ], BIGFONT, 0x00000080 );
	textline ( xmodcoords [ nxi ], 10, xmodtexts [ nxi ], BIGFONT, 0x00000080 );

	if ( secretingredient )
  {
    if ( secretsquare  && secretextracoord )
	   	textline ( 778, 10, "³", BIGFONT, 0x00000080 );
		else if ( secretsquare )
	   	textline ( 778, 10, "²", BIGFONT, 0x00000080 );
    else if ( secretextracoord )
	   	textline ( 778, 10, "¹", BIGFONT, 0x00000080 );
  }
	return;
} // printsetinfo.
/* --------------------------------------------------------------------- *
		Print info about Julia-coordinate:
 * --------------------------------------------------------------------- */
void printcoordinfo ( void )
{
  TEXTBOX ( 700, 30, 784, 94, 0x00A0A0A0, 0x00FFFFFF );

  sprintf ( stringbuf, "Preset No: %i", ( ui + 1 ) % 10 );
	textline ( 702, 30, stringbuf, BIGFONT, 0x00000080 );

	sprintf ( stringbuf, "x = %+1.9f", pcx [ ui ] );
	textline ( 703, 48, stringbuf, MEDIUMFONT, 0x00000080 );

	sprintf ( stringbuf, "y = %+1.9f", pcy [ ui ] );
	textline ( 703, 62, stringbuf, MEDIUMFONT, 0x00000080 );

	sprintf ( stringbuf, "z = %+1.9f", pcz [ ui ] );
	textline ( 703, 76, stringbuf, MEDIUMFONT, 0x00000080 );

	return;
} // printcoordinfo.
/* --------------------------------------------------------------------- *
		Print info about scene:
 * --------------------------------------------------------------------- */
void printsceneinfo ( void )
{
  TEXTBOX ( 700, 98, 784, 124, 0x00A0A0A0, 0x00FFFFFF );

  textline ( 701, 99, textbgmess [ showbackground ], SMALLFONT, 0x00000080 );
  textline ( 701, 107, textlight [ lightness ], SMALLFONT, 0x00000080 );
  textline ( 701, 115, textpales [ whitershade ], SMALLFONT, 0x00000080 );
	return;
} // printcoordinfo.
/* --------------------------------------------------------------------- *
		Clear pixel & Z-buffers:
 * --------------------------------------------------------------------- */
void clearallbufs ( long RGBdata )
{
	// Clear pixel counter for shadow map:
	shadowswritten = 0;

	// Light-source Z-buffer:
  for ( p=0; p < LHEIGHT; p++ )
  	for ( r=0; r < LWIDTH; r++ )
    	light [ p ] [ r ] = 0;

	// Image pixel & z-buffer:
	clearscreenbufs ( RGBdata );

	return;
} // clearallbufs.
/* --------------------------------------------------------------------- *
		Clear pixel & Z-buffer: (screen only, NOT shadow z-buffer)
 * --------------------------------------------------------------------- */
void clearscreenbufs ( long RGBdata )
{
	// Clear iteration and pixel counter:
	itersdone = 0;
	pixelswritten = 0;

	for ( p=0; p < BHEIGHT; p++ )
  {
  	for ( r=0; r < BWIDTH; r++ )
    {
			// pixels-buffer:
      pict [ p ] [ r ] = RGBdata;
			// View Z-buffer:
    	bpict [ p ] [ r ] = 0;
    }
  }
	return;
} // clearscreenbufs.
/* --------------------------------------------------------------------- *
		Show the mess:
 * --------------------------------------------------------------------- */
void textline ( int curposx, int curposy, char * stringdata, int fontindex, long textcolor )
{
	hdc = GetWindowDC ( hwnd );

	// Select font:
  switch ( fontindex )
  {
  	case 0:
		  SelectObject( hdc, smallfont );
    break;
  	case 1:
		  SelectObject( hdc, mediumfont );
    break;
  	case 2:
		  SelectObject( hdc, bigfont );
    break;
  } // Select font.

	// Set background mode:
	SetBkMode( hdc, TRANSPARENT );

  // Set textcolour:
	SetTextColor( hdc, textcolor );

  // create bounding box:
	tbox.left = curposx;
	tbox.top = curposy;
	tbox.right = curposx + strlen ( stringdata ) * 10;
	tbox.bottom = curposy + 16;

  DrawText	( hdc,
  						stringdata,
            	strlen ( stringdata ),
							&tbox,
    					DT_LEFT
  					);

  ReleaseDC ( hwnd, hdc );
	return;
} // textline.
/* --------------------------------------------------------------------- *
 			Initiate text output:
/* --------------------------------------------------------------------- */
void initiatetext ( void )
{
	// Setup text output: (fonts)

 	// Create fonts:
   smallfont = CreateFont	( 9,
   													0,
                            0,
 	                          0,
   	                        FW_NORMAL,
     	                      FALSE,
       	                    FALSE,
         	                  FALSE,
           	                DEFAULT_CHARSET,
             	              OUT_DEFAULT_PRECIS,
               	            CLIP_DEFAULT_PRECIS,
                 	          DEFAULT_QUALITY,
                   	        DEFAULT_PITCH,
                     	      "Verdana"
													);

   mediumfont = CreateFont	( 12,
   														0,
	                            0,
 		                          0,
   		                        FW_NORMAL,
     		                      FALSE,
       		                    FALSE,
         		                  FALSE,
           		                DEFAULT_CHARSET,
             		              OUT_DEFAULT_PRECIS,
               		            CLIP_DEFAULT_PRECIS,
                 		          DEFAULT_QUALITY,
                   		        DEFAULT_PITCH,
                     		      "Tahoma"
														);

   bigfont = CreateFont	( 15,
   												0,
                          0,
                          0,
                          FW_NORMAL,
                          FALSE,
                          FALSE,
                          FALSE,
                          DEFAULT_CHARSET,
                          OUT_DEFAULT_PRECIS,
                          CLIP_DEFAULT_PRECIS,
                          DEFAULT_QUALITY,
                          DEFAULT_PITCH,
                          "Verdana"
   											);
	return;
}// End of initiatetext.
/* --------------------------------------------------------------------- *
		Down and below: "los graphicos"!

    	First: "clean the screen" =)
 * --------------------------------------------------------------------- */
void clearscreen ( long RGBdata )
{
	int x, y, ys;
  long sc;

	for( y = ( HEIGHT - 1 ), ys = ( HEIGHT - 1 ) * lk;  y >= 0; y--, ys -= lk )
		for( x = 0,  sc = ys;  x < WIDTH; x++, sc++ )
    	lpBuf [ sc ] = RGBdata;

	return;

}// CLS.
/* --------------------------------------------------------------------- *
		picture this:
 * --------------------------------------------------------------------- */
void showpic ( void )
{
	int x, y, xx, yy;

	for( y = 0 ;  y < HEIGHT; y++ )
  {
  	yy = y << 1;
		for( x = 0;  x < WIDTH; x++ )
    {
  		xx = x << 1;

    	tcolor = pict [ yy ] [ xx ];
      tRed   = ( tcolor >> 16 ) & 0xFF;
      tGreen = ( tcolor >> 8 ) & 0xFF;
      tBlue  = tcolor & 0xFF;

    	tcolor = pict [ yy + 1 ] [ xx ];
      tRed   += ( tcolor >> 16 ) & 0xFF;
      tGreen += ( tcolor >> 8 ) & 0xFF;
      tBlue  += tcolor & 0xFF;

    	tcolor = pict [ yy + 1 ] [ xx + 1 ];
      tRed   += ( tcolor >> 16 ) & 0xFF;
      tGreen += ( tcolor >> 8 ) & 0xFF;
      tBlue  += tcolor & 0xFF;

			tcolor = pict [ yy ] [ xx + 1 ];
      tRed   += ( tcolor >> 16 ) & 0xFF;
      tGreen += ( tcolor >> 8 ) & 0xFF;
      tBlue  += tcolor & 0xFF;

      tRed = ( tRed >> 2 ) & 0xFF;
      tGreen = ( tGreen >> 2 ) & 0xFF;
      tBlue = ( tBlue >> 2 ) & 0xFF;

      tcolor = ( ( tRed << 16 ) + ( tGreen << 8 ) + tBlue ) & 0x00FFFFFF;
      lpBuf [ x + y * lk ] = tcolor;
    }
  }

	return;

}// showpic.
/* --------------------------------------------------------------------- *
		Create a palette:
 * --------------------------------------------------------------------- */
void CreatePalette ( void )
{
	bool invert, vertin, lightobject, heatvawe, sinvawe, bakwrds;
  int fade, i;
  long tc;
	float fdout, fdin, fdout2, fdin2, fdouts, fdins, ufade0, ufade1, ufade2, ufade3, ampl;
	float rf, gf, bf, freq, rl, gl, bl;

	freq = 1.0f + RND * RND * RND * 256.0f;
  rf = freq * RND * pi;
  gf = freq * RND * pi;
  bf = freq * RND * pi;

  invert = false;
  if ( RND > 0.75f )
  	invert = true;

  lightobject = false;
  if ( RND > 0.75f )
  	lightobject = true;

  vertin = false;
  if ( RND > 0.75f )
  	vertin = true;

  heatvawe = false;
  if ( RND > 0.95f )
  	heatvawe = true;

  sinvawe = false;
  if ( RND > 0.5f )
  	sinvawe = true;

	bakwrds = false;
  if ( RND > 0.5f )
  	bakwrds = true;

  fade = int ( RND * 2 );
  for ( i = 0; i != PALSIZE; i++ )
  {
  	fdout = float ( PALSIZE - i ) / PALSIZE;
		if ( bakwrds )
  		fdout = float ( i ) / PALSIZE;

    fdout2 = fdout * fdout;
    fdouts = sqrt ( fdout );

    fdin = 1.0f - fdout;
    fdin2 = 1.0f - fdouts;
    fdins = 1.0f - fdout2;

		ufade0 = fdout;
    ufade1 = fdins;
    ufade2 = fdout2;
    ufade3 = fdouts;

		if ( vertin )
     	ufade2 = 1.0f - ufade2;

		freq = rf * ufade0 * sqrt ( ufade0 );
    rl = ( 1.0f + cos ( freq ) ) * 0.5f;
    freq = gf * ufade0 * sqrt (ufade0 );
    gl = ( 1.0f + cos ( freq ) ) * 0.5f;
    freq = bf * ufade0 * sqrt ( ufade0 );
    bl = ( 1.0f + cos ( freq ) ) * 0.5f;

    if ( lightobject )
    	if ( vertin )
      {
      	length = sqrt ( rl*rl + gl*gl + bl*bl ) * 2.0f;
        rl = ( ( 1.0f + rl ) / length ) * fdin;
        gl = ( ( 1.0f + gl ) / length ) * fdin;
        bl = ( ( 1.0f + bl ) / length ) * fdin;
      }
      else
      	rl = gl = bl = fdin2;

    if ( heatvawe )
    {
    	freq = fdin2 * pii;
      rl = ( 1.0f - ( ( 1.0f + sin ( freq + rad * 240.0f ) ) * 0.5f * fdout ) ) * fdins;
      gl = ( 1.0f - ( ( 1.0f + sin ( freq + rad * 120.0f ) )  * 0.5f * fdout ) ) * fdins;
      bl = ( 1.0f - ( ( 1.0f + sin ( freq ) ) * 0.5f * fdout ) ) * fdins;
    }//heatvawe.

    if ( sinvawe )
    {
    	rl *= ( 1.0f + sin( fdout * pii * 4.1f ) ) / 2.0f;
      gl *= ( 1.0f + sin( fdout * pii * 4.2f ) ) / 2.0f;
      bl *= ( 1.0f + sin( fdout * pii * 4.3f ) ) / 2.0f;
    }//sinvawe.

    if ( invert && vertin )
    {
    	rl = ( 2.0f - rl ) / 2.0f;
      gl = ( 2.0f - gl ) / 2.0f;
      bl = ( 2.0f - bl ) / 2.0f;
		}
		else if ( invert )
		{
      rl = 1.0f - rl;
      gl = 1.0f - gl;
			bl = 1.0f - bl;
		} // Inverts.

		tc = int ( rl * float ( 0x01000000 ) ) & 0x00ff0000;
		tc += int ( gl * float ( 0x00010000 ) ) & 0x0000ff00;
		tc += int ( bl * float ( 0x00000100 ) ) & 0x000000ff;
		lpCols [ i ] = tc;
	}

	NewPalette = false;

	return;
}//Create palette.

/* --------------------------------------------------------------------- *
		View palette:
 * --------------------------------------------------------------------- */
void ShowPalette ( int mode )
{
	int 		r, p, z;
  double 	zx, zy;
  double 	adder = 4.0f / 256.0f;

	// Draw border:
	FILLBOX ( 46, 294, 306, 554, 0x00A0A0A0 );

  switch ( mode )
  {
  	case SERP:
			for ( r = ( 256 - 1 );  r >= 0; r-- )
  			for ( p = 0;  p < 256; p++ )
    			lpBuf [ 48 + p + ( HEIGHT - 304 + r ) * lk ] = lpCols [ ( int ( p * ( PALSIZE / 256 ) ) | int ( r * ( PALSIZE / 256 ) ) ) ];
    break;

  	case ABSZ:
			for( r = ( 256 - 1 ); r >= 0; r-- )
  			for ( p = 0; p < 256; p++ )
        {
          z = 1 + int ( ( sqrt ( ( r - 128 ) * ( r - 128 ) + ( p - 128 ) * ( p - 128 ) ) / 725 ) * PALSIZE );
	        z = sqrt ( z ) * sqrt ( PALSIZE );
    			lpBuf [ 48 + p + ( HEIGHT - 304 + r ) * lk ] = lpCols [ PALSIZE - ( z & ( PALSIZE - 1 ) ) ];
        }
    break;

  	case JLIA:
      zy = 2.0f;
			for( r = ( 256 - 1 ); r >= 0; r-- )
      {
	      zx = -2.0f;
  			for ( p = 0; p < 256; p++ )
        {
					x = zx;
          y = zy;
          for ( index = 128; index > 0; --index )
          {
            if ( ( x*x + y*y ) > 4.0f )
            {
            	i = 0;
            	goto quickend;
            }
          	t = x*x - y*y + pcx [ ui ];
          	y = 2 * x * y - pcy [ ui ];
            x = t;
          }
					quickend:
					lpBuf [ 48 + p + ( HEIGHT - 304 + r ) * lk ] = lpCols [ index * (PALSIZE / 128 ) ];
	        zx += adder;
        }
        zy -= adder;
      }
    break;

  	case HOTB:
		default :
    break;
	}

	return;
}// Show palette.

/* --------------------------------------------------------------------- *
		Draw a line:
 * --------------------------------------------------------------------- */
void drawLine ( void )
{
	  long double llen, ldx, ldy;
  	long int lpx, lpy, lnum;

		// clip to ( -4 / 3 ) < x < ( 4 / 3 ) / -1 < y < 1
		if (fabsl ( lxs ) > RATIO )
    {
      temp = RATIO / fabsl ( lxs );
      lxs = ( fabsl ( lxs ) * temp ) * SGN ( lxs );
      lys = ( fabsl ( lys ) * temp ) * SGN ( lys );
    }
		if ( fabsl ( lxe ) > RATIO )
    {
      temp = RATIO / fabsl ( lxe );
      lxe = ( fabsl ( lxe ) * temp ) * SGN ( lxe );
      lye = ( fabsl ( lye ) * temp ) * SGN ( lye );
    }
		if ( fabsl ( lys ) > 1.0f )
    {
      temp = 1.0f / fabsl ( lys );
      lxs = ( fabsl ( lxs ) * temp ) * SGN (lxs );
      lys = ( fabsl ( lys ) * temp ) * SGN (lys );
    }
		if ( fabsl ( lye ) > 1.0f )
    {
      temp = 1.0f / fabsl ( lye );
      lxe = ( fabsl ( lxe ) * temp ) * SGN ( lxe );
      lye = ( fabsl ( lye ) * temp ) * SGN ( lye );
    }
    // clip ends.

		ldx  = lxe - lxs;
		ldy  = lye - lys;
  	llen = sqrtl ( ldx*ldx + ldy*ldy );
	  ldx  = ldx / llen;
  	ldy  = -ldy / llen;
	  lxs  = lxs * MIDX;
	  lys  = -lys * MIDY;
	  lnum = int ( llen * MIDY ) + 1;

  	do
  	{
			lpx = int ( MIDX + lxs );
			lpy = int ( MIDY + lys );
    	lpBuf[lpx + lpy*lk] = lcol;
    	lxs += ldx;
    	lys += ldy;
		} while ( --lnum );

	return;
} // drawLine.

/* --------------------------------------------------------------------- *
		Draw a multi-colur line:
 * --------------------------------------------------------------------- */
void drawMulticolLine ( void )
{
	  long double llen, ldx, ldy;
  	long int lpx, lpy, lnum;

		// clip to ( -4 / 3 ) < x < ( 4 / 3 ) / -1 < y < 1
		if ( fabsl ( lxs ) > RATIO )
    {
      temp = RATIO / fabsl ( lxs );
      lxs = ( fabsl ( lxs ) * temp ) * SGN ( lxs );
      lys = ( fabsl ( lys ) * temp ) * SGN ( lys );
    }
		if ( fabsl ( lxe ) > RATIO )
    {
      temp = RATIO / fabsl ( lxe );
      lxe = ( fabsl ( lxe ) * temp ) * SGN ( lxe );
      lye = ( fabsl ( lye ) * temp ) * SGN ( lye );
    }
		if ( fabsl ( lys ) > 1.0f )
    {
      temp = 1.0f / fabsl ( lys );
      lxs = ( fabsl ( lxs ) * temp ) * SGN ( lxs );
      lys = ( fabsl ( lys ) * temp ) * SGN ( lys );
    }
		if ( fabsl ( lye ) > 1.0f )
    {
      temp = 1.0f / fabsl ( lye );
      lxe = ( fabsl ( lxe ) * temp ) * SGN ( lxe );
      lye = ( fabsl ( lye ) * temp ) * SGN ( lye );
    }
    // clip ends.

		ldx  = lxe - lxs;
		ldy  = lye - lys;
  	llen = sqrtl ( ldx*ldx + ldy*ldy );
	  ldx  = ldx / llen;
  	ldy  = -ldy / llen;
	  lxs  = lxs * MIDY;
	  lys  = -lys * MIDY;
	  lnum = int ( llen * MIDY ) + 1;

  	do
  	{
			lpx = int ( MIDX + lxs );
			lpy = int ( MIDY + lys );
    	lpBuf [ lpx + lpy*lk ] = lpCols [ lnum & ( PALSIZE - 1 ) ];
    	lxs += ldx;
    	lys += ldy;
		} while ( --lnum );

	return;
} // drawMulticolLine.
/* --------------------------------------------------------------------- *
		Draw a box from floats: (hi-tech mode =)
 * --------------------------------------------------------------------- */
void drawBox ( void )
{
  	long int lpx, lpy, lnx, lny;

		// clip to ( -4 /  3) < x < ( 4 / 3 ) / -1 < y < 1
		if ( fabsl ( lxs ) > RATIO )
    {
      temp = RATIO / fabsl ( lxs );
      lxs = ( fabsl ( lxs ) * temp ) * SGN ( lxs );
      lys = ( fabsl ( lys ) * temp ) * SGN ( lys );
    }
		if ( fabsl ( lxe ) > RATIO )
    {
      temp = RATIO / fabsl ( lxe );
      lxe = ( fabsl ( lxe ) * temp ) * SGN ( lxe );
      lye = ( fabsl ( lye ) * temp ) * SGN ( lye );
    }
		if ( fabsl ( lys ) > 1.0f )
    {
      temp = 1.0f / fabsl( lys );
      lxs = ( fabsl ( lxs ) * temp ) * SGN ( lxs );
      lys = ( fabsl ( lys ) * temp ) * SGN ( lys );
    }
		if ( fabsl ( lye ) > 1.0f )
    {
      temp = 1.0f / fabsl ( lye );
      lxe = ( fabsl ( lxe ) * temp ) * SGN ( lxe );
      lye = ( fabsl ( lye ) * temp ) * SGN ( lye );
    }
    // clip ends.

    if ( lxs > lxe )
			temp = lxs, lxs  = lxe, lxe = temp;
    if ( lys > lye )
			temp = lys, lys  = lye, lye = temp;

		lnx  = int ( ( lxe - lxs ) * MIDY );
		lny  = int ( ( lye - lys ) * MIDY );
		lpx = MIDX + int ( MIDY * lxs );
		lpy = MIDY - int ( MIDY * lys );

		do
    {
			for ( index = 0; index < lnx; index++ )
    		lpBuf [ index+lpx + lpy*lk ] = lcol;
      --lpy;
    } while ( --lny != 0 );

	return;
} // drawBox.
/* --------------------------------------------------------------------- *
		Draw a box: (from ints)
 * --------------------------------------------------------------------- */
void drawBoxi ( void )
{
  	int boxw, boxh;

		// clip:
		if ( lixs < 0 )
      lixs = 0;
		if ( lixs >= WIDTH )
      lixs = WIDTH - 1;

		if ( lixe < 0 )
      lixe = 0;
		if ( lixe >= WIDTH )
      lixe = WIDTH - 1;

		if ( liys < 0 )
      liys = 0;
		if ( liys >= HEIGHT )
      liys = HEIGHT - 1;

		if ( liye < 0 )
      liye = 0;
		if ( liye >= HEIGHT )
      liye = HEIGHT - 1;
    // clip ends.

		// Swap coords?
    if ( lixs > lixe )
    {
			tmp = lixs;
      lixs = lixe;
      lixe = tmp;
    }
    if ( liys > liye )
    {
			tmp = liys;
      liys = liye;
      liye = tmp;
    }

		// Calculate with & height of box:
		boxw  = lixe - lixs;
		boxh  = liye - liys;

		do
    {
			for ( index = 0; index < boxw; index++ )
    		lpBuf [ index + lixs + liys * lk ] = lcol;
      liys++;
    } while ( --boxh > 0 );

	return;
} // drawBoxi.


















