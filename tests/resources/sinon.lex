entier	mot_clef	entier
$a	identificateur	$a
;	symbole	POINT_VIRGULE
main	identificateur	main
(	symbole	PARENTHESE_OUVRANTE
)	symbole	PARENTHESE_FERMANTE
{	symbole	ACCOLADE_OUVRANTE
$a	identificateur	$a
=	symbole	EGAL
lire	mot_clef	lire
(	symbole	PARENTHESE_OUVRANTE
)	symbole	PARENTHESE_FERMANTE
;	symbole	POINT_VIRGULE
si	mot_clef	si
(	symbole	PARENTHESE_OUVRANTE
$a	identificateur	$a
)	symbole	PARENTHESE_FERMANTE
alors	mot_clef	alors
{	symbole	ACCOLADE_OUVRANTE
ecrire	mot_clef	ecrire
(	symbole	PARENTHESE_OUVRANTE
1	nombre	1
)	symbole	PARENTHESE_FERMANTE
;	symbole	POINT_VIRGULE
}	symbole	ACCOLADE_FERMANTE
sinon	mot_clef	sinon
{	symbole	ACCOLADE_OUVRANTE
ecrire	mot_clef	ecrire
(	symbole	PARENTHESE_OUVRANTE
0	nombre	0
)	symbole	PARENTHESE_FERMANTE
;	symbole	POINT_VIRGULE
}	symbole	ACCOLADE_FERMANTE
}	symbole	ACCOLADE_FERMANTE
