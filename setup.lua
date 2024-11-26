function bind_args(f,...)
  local args = {...}
  return function() f(table.unpack(args)) end
end

--be careful with these!
chords[" "] = play
chords["l"] = toggle_loop
chords["<SHFT>;"] = activate_cmd_line

chords["<LA>"] = function(x,y,w,h) step_cursor(-1,w) end
chords["<RA>"] = function(x,y,w,h) step_cursor(1,w) end
chords["<CMD><LA>"] = function(x,y,w,h) set_cursor(previous_mark(cursor() or 0.0)) end
chords["<CMD><RA>"] = function(x,y,w,h) set_cursor(next_mark(cursor() or 0.0)) end

chords["<SHFT><LA>"] = function(x,y,w,h) feather_selection(-1,w) end
chords["<SHFT><RA>"] = function(x,y,w,h) feather_selection(1,w) end

chords["<CMD><SHFT><LA>"] = function(x,y,w,h) expand_left() end
chords["<CMD><SHFT><RA>"] = function(x,y,w,h) expand_right() end

chords["<CMD><DN>"] = bind_args(zoom_out)
chords["<CMD><UP>"] = bind_args(zoom_selected)

chords["<ESC>"] = function() clear_cursor() end

chords["<CMD>rs"] = function() set_ruler(cursor(),selection()) end
chords["<CMD>rg"] = bind_args(scale_ruler,0.5)
chords["<CMD>rt"] = bind_args(scale_ruler,2)
chords["<CMD>rx"] = bind_args(scale_ruler,0.0)
chords["zc"] = function(x,y,w,h) toggle_over(y) end

copy_buffer = nil

chords["<CMD>c"] = function(x,y,w,h) 
  copy_buffer = copy_snd()
end

chords["<CMD>v"] = function(x,y,w,h)
  if copy_buffer ~= nil then
    paste(copy_buffer)
  end
end

chords["<CMD>z"] = undo
chords["<BKSP>"] = delete

click_modes["<SHFT>"] = function(x,y,w,h)
  local cursor_pt = cursor() or 0.0
  local click_pt = x * snd_len()
  
  select_region(cursor_pt,click_pt)
end

drag_modes["<SHFT>"] = function(x,y,w,h)
  local cursor_pt = cursor() or 0.0
  local click_pt = x * snd_len()
  
  select_region(cursor_pt,click_pt)
end

click_modes["s"] = function(x,y,w,h)
  local click_pt = x * snd_len()
  set_cursor(previous_mark(click_pt))
end

drag_modes["s"] = function(x,y,w,h)
  local pt = x * snd_len()
  local start = cursor() or previous_mark(pt)
  select_region(start,nearest_mark(pt))
end

function slide_selection(x)
  local snd_len = snd_len()
  local pt = x * snd_len 
  local len = selection()

  if len == nil then return end
  local clamped_pt = pt;
  if len >= 0 then
    clamped_pt = math.min(pt,snd_len - len)
  else 
    clamped_pt = math.max(-len,pt)
  end
  
  set_cursor(clamped_pt)
  select_len(len)

end

click_modes["d"] = slide_selection
drag_modes["d"] = slide_selection
