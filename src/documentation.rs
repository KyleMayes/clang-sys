// Copyright 2016 Kyle Mayes
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::mem;

use libc::{c_uint, c_void};

use super::{CXCursor, CXString, CXTranslationUnit};

//================================================
// Enums
//================================================

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum CXCommentInlineCommandRenderKind {
    Normal = 0,
    Bold = 1,
    Monospaced = 2,
    Emphasized = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum CXCommentKind {
    Null = 0,
    Text = 1,
    InlineCommand = 2,
    HTMLStartTag = 3,
    HTMLEndTag = 4,
    Paragraph = 5,
    BlockCommand = 6,
    ParamCommand = 7,
    TParamCommand = 8,
    VerbatimBlockCommand = 9,
    VerbatimBlockLine = 10,
    VerbatimLine = 11,
    FullComment = 12,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum CXCommentParamPassDirection {
    In = 0,
    Out = 1,
    InOut = 2,
}

//================================================
// Structs
//================================================

// Transparent ___________________________________

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct CXComment {
    pub ASTNode: *const c_void,
    pub TranslationUnit: CXTranslationUnit,
}

default!(CXComment);

//================================================
// Functions
//================================================

extern {
    pub fn clang_BlockCommandComment_getArgText(comment: CXComment, index: c_uint) -> CXString;
    pub fn clang_BlockCommandComment_getCommandName(comment: CXComment) -> CXString;
    pub fn clang_BlockCommandComment_getNumArgs(comment: CXComment) -> c_uint;
    pub fn clang_BlockCommandComment_getParagraph(comment: CXComment) -> CXComment;
    pub fn clang_Comment_getChild(comment: CXComment, index: c_uint) -> CXComment;
    pub fn clang_Comment_getKind(comment: CXComment) -> CXCommentKind;
    pub fn clang_Comment_getNumChildren(comment: CXComment) -> c_uint;
    pub fn clang_Comment_isWhitespace(comment: CXComment) -> c_uint;
    pub fn clang_Cursor_getParsedComment(C: CXCursor) -> CXComment;
    pub fn clang_FullComment_getAsHTML(comment: CXComment) -> CXString;
    pub fn clang_FullComment_getAsXML(comment: CXComment) -> CXString;
    pub fn clang_HTMLStartTagComment_isSelfClosing(comment: CXComment) -> c_uint;
    pub fn clang_HTMLStartTag_getAttrName(comment: CXComment, index: c_uint) -> CXString;
    pub fn clang_HTMLStartTag_getAttrValue(comment: CXComment, index: c_uint) -> CXString;
    pub fn clang_HTMLStartTag_getNumAttrs(comment: CXComment) -> c_uint;
    pub fn clang_HTMLTagComment_getAsString(comment: CXComment) -> CXString;
    pub fn clang_HTMLTagComment_getTagName(comment: CXComment) -> CXString;
    pub fn clang_InlineCommandComment_getArgText(comment: CXComment, index: c_uint) -> CXString;
    pub fn clang_InlineCommandComment_getCommandName(comment: CXComment) -> CXString;
    pub fn clang_InlineCommandComment_getNumArgs(comment: CXComment) -> c_uint;
    pub fn clang_InlineCommandComment_getRenderKind(comment: CXComment) -> CXCommentInlineCommandRenderKind;
    pub fn clang_InlineContentComment_hasTrailingNewline(comment: CXComment) -> c_uint;
    pub fn clang_ParamCommandComment_getDirection(comment: CXComment) -> CXCommentParamPassDirection;
    pub fn clang_ParamCommandComment_getParamIndex(comment: CXComment) -> c_uint;
    pub fn clang_ParamCommandComment_getParamName(comment: CXComment) -> CXString;
    pub fn clang_ParamCommandComment_isDirectionExplicit(comment: CXComment) -> c_uint;
    pub fn clang_ParamCommandComment_isParamIndexValid(comment: CXComment) -> c_uint;
    pub fn clang_TParamCommandComment_getDepth(comment: CXComment) -> c_uint;
    pub fn clang_TParamCommandComment_getIndex(comment: CXComment, depth: c_uint) -> c_uint;
    pub fn clang_TParamCommandComment_getParamName(comment: CXComment) -> CXString;
    pub fn clang_TParamCommandComment_isParamPositionValid(comment: CXComment) -> c_uint;
    pub fn clang_TextComment_getText(comment: CXComment) -> CXString;
    pub fn clang_VerbatimBlockLineComment_getText(comment: CXComment) -> CXString;
    pub fn clang_VerbatimLineComment_getText(comment: CXComment) -> CXString;
}
