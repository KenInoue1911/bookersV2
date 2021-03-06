class ApplicationController < ActionController::Base
  before_action :configure_permitted_parameters, if: :devise_controller?
  def after_sign_in_path_for(resource)
    user_path(current_user.id)
    if current_user
        flash[:notice]
        user_path(current_user.id)
    else
        flash[:notice]
        user_path(current_user.id)
    end
  end
  
  def after_sign_out_path_for(resource)
    "/"
  end

  private

    def configure_permitted_parameters
      devise_parameter_sanitizer.permit(:sign_up,keys:[:email])
    end

end