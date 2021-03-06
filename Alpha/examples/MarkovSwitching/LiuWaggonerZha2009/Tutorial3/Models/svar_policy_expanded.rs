// svar_core.rs line 8
endogenous	 Y, "Output gap", PAI, "Inflation", I, "Fed Funds rate"
exogenous EPAI,  "Phil. curve shock", EY, "IS curve shock", EI, "Taylor rule shock"
parameters alpha_pai1, "$\alpha_{\pi,1}$", alpha_pai2, "$\alpha_{\pi,2}$", alpha_y, "$\alpha_{y}$", c_pai, "$c_{\pi}$",
c_y, "$c_{y}$", beta_y1, "$\beta_{y,1}$", beta_y2, "$\beta_{y,2}$", beta_r, "$\beta_{r}$",
observables I, Y, PAI
model(linear)
    # alpha_pai = 1/sig_pai;
    # beta_y    = 1/sig_y;
    # gam_i     = 1/sig_i;
   alpha_pai*PAI   = c_pai + alpha_pai1*PAI{-1} + alpha_pai2*PAI{-2} +alpha_y*Y{-1} + EPAI;
   beta_y*Y = c_y + beta_y1*Y{-1} + beta_y2*Y{-2} -beta_r*(I{-1}-PAI{-1}) + EY;
   gam_i*I = c_i + gam_i*rho_i*I{-1} +gam_i*(1-rho_i)*(gam_y*Y+gam_pai*PAI) + EI;
steady_state_model(imposed)
	xx_ssmdef_1=alpha_pai-alpha_pai1-alpha_pai2;
	xx_ssmdef_2=gam_i*(1-rho_i);
	xx_ssmdef_3=beta_r*(gam_pai-1);
	xx_ssmdef_4=c_y-beta_r*c_i/xx_ssmdef_2-xx_ssmdef_3*c_pai/xx_ssmdef_1;
	xx_ssmdef_5=beta_y-beta_y1-beta_y2+beta_r*gam_y+xx_ssmdef_3*alpha_y/xx_ssmdef_1;
	Y=xx_ssmdef_4/xx_ssmdef_5;
	PAI=c_pai/xx_ssmdef_1+alpha_y*Y/xx_ssmdef_1;
	I=c_i/xx_ssmdef_2+gam_y*Y+gam_pai*PAI;
parameterization
    alpha_pai1, 	0.9, 	0.05, 	1.5, 	gamma_pdf(0.9);
    alpha_pai2, 	0  , 	-1  , 	1  , 	normal_pdf(0.9); 
    alpha_y,    	0.1, 	0.05, 	1.5, 	gamma_pdf(0.9);  
    c_pai,      	0  , 	-1  , 	1  , 	normal_pdf(0.9);  
    c_y,        	0  , 	-1  , 	1  , 	normal_pdf(0.9);
    beta_y1,    	0.9, 	0.1 , 	1.5, 	gamma_pdf(0.9);
    beta_y2,    	0  , 	-2  , 	2  , 	normal_pdf(0.9);
    beta_r,     	0.1, 	0.05, 	1  , 	gamma_pdf(0.9); 
// constant_volatility.rs line 1
parameters sig_pai, "$\sigma_{\pi}$",	sig_y, "$\sigma_{y}$", sig_i, "$\sigma_{i}$"
parameterization
	sig_pai,   		0.1, 	0.05, 	  3, 	weibull_pdf(0.9); 
	sig_y,  		0.1, 	0.05, 	  3, 	weibull_pdf(0.9);  
	sig_i, 			0.1, 	0.05,     3, 	weibull_pdf(0.9); 
// switching_policy.rs line 1
parameters pol_tp_1_2 pol_tp_2_1
parameters(pol,2) rho_i, "$\rho_{i}$" gam_y, "$\gamma_{y}$" gam_pai, "$\gamma_{\pi}$" c_i, "$c_{i}$"
parameterization
	rho_i(pol,1), 			0.6, 	0.1, 	0.7, 	beta_pdf(0.9);
	rho_i(pol,2), 			0.6, 	0.1, 	0.7, 	beta_pdf(0.9);
	gam_y(pol,1), 			0.5, 	0.1, 	1.5, 	gamma_pdf(0.9);
	gam_y(pol,2), 			0.5, 	0.1, 	1.5, 	gamma_pdf(0.9);
	gam_pai(pol,1), 		1.5, 	0.5, 	3, 		gamma_pdf(0.9);
	gam_pai(pol,2), 		1.0, 	0.5, 	3, 		gamma_pdf(0.9);
	c_i(pol,1), 			0  , 	-1, 	1, 		normal_pdf(0.9); 
	c_i(pol,2), 			0  , 	-1, 	1, 		normal_pdf(0.9);
	pol_tp_1_2,   			0.15,	0.1, 	0.5, 	beta_pdf(0.9);
	pol_tp_2_1,   			0.15,	0.1, 	0.5, 	beta_pdf(0.9);
parameter_restrictions
	gam_pai(pol,1)>=gam_pai(pol,2);	
