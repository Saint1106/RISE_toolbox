function preserve(varargin)
wh=evalin('base','who');
nvar=numel(varargin);
discard=nan(nvar,1);
for jj=1:nvar
    vv=varargin{jj};
    loc=find(strcmp(vv,wh));
    if ~isempty(loc)
        discard(jj)=loc;
    end
end
discard=discard(~isnan(discard));
wh(discard)=[];
for jj=1:numel(wh)
    evalin('base',['clear(''',wh{jj},''')'])
end