Rails.application.routes.draw do
  devise_for :users
  root to: 'home#top'
  resources :home do
  collection do
    get :about
  end
end
   resources :books, only: [:new, :create, :index, :show, :destroy, :edit, :update]
  # For details on the DSL available within this file, see http://guides.rubyonrails.org/routing.html
  resources :users, only: [:show, :edit, :update, :index, :create]
end
