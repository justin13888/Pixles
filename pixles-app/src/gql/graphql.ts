/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  /**
   * Implement the DateTime<Utc> scalar
   *
   * The input/output is a string in RFC3339 format.
   */
  DateTime: { input: any; output: any; }
};

export type Admin = {
  __typename?: 'Admin';
  email: Scalars['String']['output'];
  id: Scalars['ID']['output'];
  name: Scalars['String']['output'];
};

export type AdminMutation = {
  __typename?: 'AdminMutation';
  createAdmin: Admin;
  updateAdmin: Admin;
};


export type AdminMutationCreateAdminArgs = {
  input: CreateAdminInput;
};


export type AdminMutationUpdateAdminArgs = {
  id: Scalars['ID']['input'];
  input: UpdateAdminInput;
};

export type AdminQuery = {
  __typename?: 'AdminQuery';
  getAdmin: Admin;
  listAdmins: Array<Admin>;
};


export type AdminQueryGetAdminArgs = {
  id: Scalars['ID']['input'];
};

/** The response from the login user mutation */
export type AuthResponse = {
  __typename?: 'AuthResponse';
  token: Scalars['String']['output'];
  user?: Maybe<User>;
};

export type CreateAdminInput = {
  email: Scalars['String']['input'];
  name: Scalars['String']['input'];
};

export type CreateMediaInput = {
  email: Scalars['String']['input'];
  name: Scalars['String']['input'];
};

export type LoginUserInput = {
  email: Scalars['String']['input'];
  password: Scalars['String']['input'];
};

export type Media = {
  __typename?: 'Media';
  email: Scalars['String']['output'];
  id: Scalars['ID']['output'];
  name: Scalars['String']['output'];
};

export type MediaMutation = {
  __typename?: 'MediaMutation';
  createMedia: Media;
  updateMedia: Media;
};


export type MediaMutationCreateMediaArgs = {
  input: CreateMediaInput;
};


export type MediaMutationUpdateMediaArgs = {
  id: Scalars['ID']['input'];
  input: UpdateMediaInput;
};

export type MediaQuery = {
  __typename?: 'MediaQuery';
  getMedia: Media;
  listMedias: Array<Media>;
};


export type MediaQueryGetMediaArgs = {
  id: Scalars['ID']['input'];
};

export type MutationRoot = {
  __typename?: 'MutationRoot';
  admin: AdminMutation;
  media: MediaMutation;
  user: UserMutation;
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  admin: AdminQuery;
  media: MediaQuery;
  user: UserQuery;
};

export type RegisterUserInput = {
  email: Scalars['String']['input'];
  name: Scalars['String']['input'];
  password: Scalars['String']['input'];
};

/** The response from the register user mutation */
export type RegisterUserResponse = {
  __typename?: 'RegisterUserResponse';
  data?: Maybe<AuthResponse>;
  errors: Array<Scalars['String']['output']>;
  success: Scalars['Boolean']['output'];
};

export type UpdateAdminInput = {
  email?: InputMaybe<Scalars['String']['input']>;
  name?: InputMaybe<Scalars['String']['input']>;
};

export type UpdateMediaInput = {
  email?: InputMaybe<Scalars['String']['input']>;
  name?: InputMaybe<Scalars['String']['input']>;
};

export type User = {
  __typename?: 'User';
  accountVerified: Scalars['Boolean']['output'];
  createdAt: Scalars['DateTime']['output'];
  deletedAt?: Maybe<Scalars['DateTime']['output']>;
  email: Scalars['String']['output'];
  id: Scalars['String']['output'];
  modifiedAt: Scalars['DateTime']['output'];
  name: Scalars['String']['output'];
  needsOnboarding: Scalars['Boolean']['output'];
  username: Scalars['String']['output'];
};

export type UserMutation = {
  __typename?: 'UserMutation';
  login: AuthResponse;
  register: RegisterUserResponse;
};


export type UserMutationLoginArgs = {
  input: LoginUserInput;
};


export type UserMutationRegisterArgs = {
  input: RegisterUserInput;
};

export type UserQuery = {
  __typename?: 'UserQuery';
  getUser: User;
  listUsers: Array<User>;
  me?: Maybe<User>;
};


export type UserQueryGetUserArgs = {
  id: Scalars['ID']['input'];
};

export type FooQueryVariables = Exact<{
  id: Scalars['ID']['input'];
}>;


export type FooQuery = { __typename?: 'QueryRoot', user: { __typename?: 'UserQuery', getUser: { __typename?: 'User', id: string, username: string, name: string } } };

export type BarMutationVariables = Exact<{ [key: string]: never; }>;


export type BarMutation = { __typename?: 'MutationRoot', user: { __typename?: 'UserMutation', register: { __typename?: 'RegisterUserResponse', success: boolean, errors: Array<string>, data?: { __typename?: 'AuthResponse', token: string, user?: { __typename?: 'User', id: string, username: string } | null } | null } } };


export const FooDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"foo"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"id"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"user"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"getUser"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"id"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}}]}}]} as unknown as DocumentNode<FooQuery, FooQueryVariables>;
export const BarDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"bar"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"user"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"register"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"name"},"value":{"kind":"StringValue","value":"hi","block":false}},{"kind":"ObjectField","name":{"kind":"Name","value":"email"},"value":{"kind":"StringValue","value":"hi","block":false}},{"kind":"ObjectField","name":{"kind":"Name","value":"password"},"value":{"kind":"StringValue","value":"foo","block":false}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"success"}},{"kind":"Field","name":{"kind":"Name","value":"data"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"token"}},{"kind":"Field","name":{"kind":"Name","value":"user"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"username"}}]}}]}},{"kind":"Field","name":{"kind":"Name","value":"errors"}}]}}]}}]}}]} as unknown as DocumentNode<BarMutation, BarMutationVariables>;