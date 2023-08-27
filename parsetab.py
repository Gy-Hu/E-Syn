
# parsetab.py
# This file is automatically generated. Do not edit.
# pylint: disable=W,C,R
_tabversion = '3.10'

_lr_method = 'LALR'

_lr_signature = 'leftORleftANDrightNOTAND LPAREN NOT OR RPAREN SYMBOLprop : termprop : LPAREN AND prop prop RPARENprop : LPAREN OR prop prop RPARENprop : LPAREN NOT prop RPARENterm : idid : SYMBOL'
    
_lr_action_items = {'LPAREN':([0,2,4,5,6,7,8,9,10,14,15,16,],[3,-1,-5,-6,3,3,3,3,3,-4,-2,-3,]),'SYMBOL':([0,2,4,5,6,7,8,9,10,14,15,16,],[5,-1,-5,-6,5,5,5,5,5,-4,-2,-3,]),'$end':([1,2,4,5,14,15,16,],[0,-1,-5,-6,-4,-2,-3,]),'RPAREN':([2,4,5,11,12,13,14,15,16,],[-1,-5,-6,14,15,16,-4,-2,-3,]),'AND':([3,],[6,]),'OR':([3,],[7,]),'NOT':([3,],[8,]),}

_lr_action = {}
for _k, _v in _lr_action_items.items():
   for _x,_y in zip(_v[0],_v[1]):
      if not _x in _lr_action:  _lr_action[_x] = {}
      _lr_action[_x][_k] = _y
del _lr_action_items

_lr_goto_items = {'prop':([0,6,7,8,9,10,],[1,9,10,11,12,13,]),'term':([0,6,7,8,9,10,],[2,2,2,2,2,2,]),'id':([0,6,7,8,9,10,],[4,4,4,4,4,4,]),}

_lr_goto = {}
for _k, _v in _lr_goto_items.items():
   for _x, _y in zip(_v[0], _v[1]):
       if not _x in _lr_goto: _lr_goto[_x] = {}
       _lr_goto[_x][_k] = _y
del _lr_goto_items
_lr_productions = [
  ("S' -> prop","S'",1,None,None,None),
  ('prop -> term','prop',1,'p_prop_term','to_sympy_parser_sexpr.py',30),
  ('prop -> LPAREN AND prop prop RPAREN','prop',5,'p_prop_and','to_sympy_parser_sexpr.py',34),
  ('prop -> LPAREN OR prop prop RPAREN','prop',5,'p_prop_or','to_sympy_parser_sexpr.py',38),
  ('prop -> LPAREN NOT prop RPAREN','prop',4,'p_prop_not','to_sympy_parser_sexpr.py',42),
  ('term -> id','term',1,'p_term_id','to_sympy_parser_sexpr.py',46),
  ('id -> SYMBOL','id',1,'p_id_symbol','to_sympy_parser_sexpr.py',50),
]
