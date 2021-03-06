%% Code automagically generated by rise on 21-Jul-2013 16:12:48

function [G0,retcode]=transition_matrix(y,x,ss,param,def,s0,s1)

retcode=0;
if nargout>0;
G0=sparse(4,4);
ref_0_1=exp(y(8));
ref_0_2=ref_0_1*param(7);
ref_0_3=ref_0_2+param(6);
ref_0_4=exp(ref_0_3);
ref_0_5=1+ref_0_4;
ref_0_6=ref_0_4/ref_0_5;
ref_0_7=1-ref_0_6;
ref_0_8=1-ref_0_7;
ref_0_9=1-param(32);
ref_0_10=1-param(33);
G0(1,1)=ref_0_7*ref_0_9;
G0(1,2)=ref_0_7*param(32);
G0(1,3)=ref_0_6*ref_0_9;
G0(1,4)=ref_0_6*param(32);
G0(2,1)=ref_0_7*param(33);
G0(2,2)=ref_0_7*ref_0_10;
G0(2,3)=ref_0_6*param(33);
G0(2,4)=ref_0_6*ref_0_10;
G0(3,3)=ref_0_8*ref_0_9;
G0(3,4)=ref_0_8*param(32);
G0(4,3)=ref_0_8*param(33);
G0(4,4)=ref_0_8*ref_0_10;
G0(3,1)=G0(1,1);
G0(3,2)=G0(1,2);
G0(4,1)=G0(2,1);
G0(4,2)=G0(2,2);
end;
if any(any(isnan(Q))) || any(any(Q<0)) || any(any(Q>1));
Q=[];
retcode=3;
end;
